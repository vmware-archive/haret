// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::io::{self, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::fs::File;
use std::str::FromStr;
use slog::Logger;
use rabble::{self, Node, Pid, Envelope, CorrelationId, Result, ServiceHandler};
use amy::Registrar;
use msg::Msg;
use disk_msgs::{DiskReq, DiskRpy};

/// The disk manager is in charge of reading and writing all data to disk
///
/// It runs in its own thread to prevent the need to run blocking operations in lightweight
/// processes that would block other processes in the executor.
pub struct DiskMgr {
    // The directory where all data is written
    pub path: PathBuf,
    pub pid: Pid,
    node: Node<Msg>,
    #[allow(unused)]
    logger: Logger
}

impl DiskMgr {
    pub fn new(node: Node<Msg>, mut dir: PathBuf, logger: Logger) -> DiskMgr {
        let pid = Pid {
            group: None,
            name: "disk_mgr".to_owned(),
            node: node.id.clone()
        };
        dir.push("nonce");
        DiskMgr {
            path: dir,
            pid: pid,
            node: node,
            logger: logger
        }
    }

    fn write_nonce(&mut self, filename: String, nonce: u64) {
        // We purposefully unwrap in this clause because disk errors should be fatal
        self.path.push(&filename);
        let mut file = File::create(&self.path).unwrap();
        // Write the u64 nonce as a string
        file.write_all(nonce.to_string().as_bytes()).unwrap();
        file.sync_all().unwrap();
        let _ = self.path.pop();
    }

    fn read_nonce(&mut self, filename: String) -> io::Result<u64> {
        self.path.push(&filename);
        File::open(&self.path).map(|mut file| {
            let mut numstr = String::new();
            // Treat failure to read as fatal
            file.read_to_string(&mut numstr).unwrap();
            // Treat file corruption as fatal
            u64::from_str(&numstr).unwrap()
        })
    }

    fn send_read_result(&mut self,
                        nonce: io::Result<u64>,
                        to: Pid,
                        cid: Option<CorrelationId>) -> Result<()>
    {
        match nonce {
            Ok(nonce) => {
                self.write_nonce(to.to_string(), nonce + 1);
                self.send_nonce(nonce, to, cid)
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    self.write_nonce(to.to_string(), 1);
                    return self.send_not_found(to, cid);
                }
                panic!("Failed to read nonce: {}", e);
            }
        }
    }

    fn send_nonce(&self, nonce: u64, to: Pid, cid: Option<CorrelationId>) -> Result<()> {
        let msg = DiskRpy::Nonce(nonce).into();
        let envelope = Envelope::new(to, self.pid.clone(), msg, cid);
        self.node.send(envelope).map_err(|e| e.into())
    }

    fn send_not_found(&self, to: Pid, cid: Option<CorrelationId>) -> Result<()> {
        let msg = DiskRpy::NotFound.into();
        let envelope = Envelope::new(to, self.pid.clone(), msg, cid);
        self.node.send(envelope).map_err(|e| e.into())
    }
}

impl ServiceHandler<Msg> for DiskMgr {
    fn handle_envelope(&mut self,
                       _: &Node<Msg>,
                       envelope: Envelope<Msg>,
                       _: &Registrar) -> Result<()>
    {
        let Envelope {from, msg, correlation_id, ..} = envelope;
        match msg {
            rabble::Msg::User(Msg::DiskReq(DiskReq::ReadNonce)) => {
                let nonce = self.read_nonce(from.to_string());
                self.send_read_result(nonce, from, correlation_id)
            }
            _ => unreachable!()
        }
    }
}
