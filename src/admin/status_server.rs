// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashMap, HashSet};
use time::SteadyTime;
use slog::Logger;
use rabble::{self, Pid, Node, Envelope, CorrelationId, ServiceHandler};
use rabble::StatusTable;
use amy::Registrar;
use msg::Msg;
use admin::{AdminReq, AdminRpy};

const TIMEOUT: usize = 2000; // milliseconds

/// Manage all status information for a node.
///
/// Each subsystem's status is maintained in a seperate HashMap.
pub struct StatusServer {
    pid: Pid,
    executor: Pid,
    node: Node<Msg>,
    logger: Logger,
    updated: SteadyTime,
    tables: HashMap<String, StatusTable>,
    waiting: Vec<CorrelationId>,
    correlation_id: CorrelationId,
    request_in_flight: bool,
    received_components: HashSet<String>,
    all_components: HashSet<String>
}

impl StatusServer {
    pub fn new(node: Node<Msg>,
               executor: Pid,
               logger: Logger) -> StatusServer {
        let pid = Pid {
            group: None,
            name: "status_server".to_string(),
            node: node.id.clone()
        };
        StatusServer {
            pid: pid.clone(),
            executor: executor,
            node: node,
            logger: logger,
            updated: SteadyTime::now(),
            tables: HashMap::new(),
            waiting: Vec::new(),
            correlation_id: CorrelationId {
                pid: pid,
                connection: None,
                request: Some(0)
            },
            request_in_flight: false,
            received_components: HashSet::new(),
            all_components: [
                "cluster_server",
                "executor"
            ].iter().map(|s| s.to_string()).collect()
        }
    }

    fn request_status(&mut self, c_id: CorrelationId) {
        self.waiting.push(c_id);
        if !self.request_in_flight {
            self.start_request();
            self.send_status_requests();
        }
    }

    fn start_request(&mut self) {
        self.correlation_id.request = Some(self.correlation_id.request.unwrap() + 1);
        self.request_in_flight = true;
        self.start_timer();
    }

    fn stop_request(&mut self) {
        if self.request_in_flight {
            self.request_in_flight = false;
            self.updated = SteadyTime::now();
            self.cancel_timer();
        }
    }

    fn start_timer(&mut self) {
        let envelope = Envelope {
            to: self.executor.clone(),
            from: self.pid.clone(),
            msg: rabble::Msg::StartTimer(TIMEOUT),
            correlation_id: Some(self.correlation_id.clone())
        };
        self.node.send(envelope).unwrap();
    }

    fn cancel_timer(&mut self) {
        let envelope = Envelope {
            to: self.executor.clone(),
            from: self.pid.clone(),
            msg: rabble::Msg::CancelTimer(Some(self.correlation_id.clone())),
            correlation_id: Some(self.correlation_id.clone())
        };
        self.node.send(envelope).unwrap();
    }

    fn send_status_requests(&mut self) {
        self.node.cluster_status(self.correlation_id.clone()).unwrap();
        self.node.executor_status(self.correlation_id.clone()).unwrap();
    }

    fn handle_status_response(&mut self,
                              component: String,
                              status: StatusTable,
                              c_id: CorrelationId) 
    {
        if c_id == self.correlation_id {
            self.received_components.insert(component.clone());
            self.tables.insert(component, status);
        }

        if self.all_components == self.received_components {
            self.stop_request();
            self.send_replies();
        }
    }

    fn send_replies(&mut self) {
        for c_id in self.waiting.drain(..) {
            let reply = Msg::AdminRpy(AdminRpy::Status(self.tables.clone()));
            let envelope = Envelope {
                to: self.correlation_id.pid.clone(),
                from: self.pid.clone(),
                msg: rabble::Msg::User(reply),
                correlation_id: Some(c_id)
            };
            self.node.send(envelope).unwrap();
        }
    }

    fn handle_timeout(&mut self) {
        self.stop_request();
        self.send_replies();
    }
}


impl ServiceHandler<Msg> for StatusServer {
    fn handle_envelope(&mut self,
                       _: &Node<Msg>,
                       envelope: Envelope<Msg>,
                       _: &Registrar) -> rabble::Result<()>
    {
        let Envelope{msg, correlation_id, ..} = envelope;
        match msg {
            rabble::Msg::User(Msg::AdminReq(AdminReq::GetStatus)) => {
                if self.tables.len() == 0 ||
                    (SteadyTime::now() - self.updated).num_seconds() > 10
                {
                    self.request_status(correlation_id.unwrap());
                }
            },
            rabble::Msg::User(Msg::AdminRpy(AdminRpy::StatusTable(component, status))) => {
                self.handle_status_response(component, status, correlation_id.unwrap());
            },
            rabble::Msg::Timeout => self.handle_timeout(),
            _ => unreachable!()
        }
        Ok(())
    }
}
