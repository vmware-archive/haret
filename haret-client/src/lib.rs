// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate haret;
extern crate protobuf;

use std::io::{Read, Result, Error, ErrorKind, Write};
use std::mem;
use std::net::TcpStream;
use protobuf::{RepeatedField, parse_from_bytes, Message};
use haret::api::messages::*;

/// This struct represents the Haret client implementation in rust. It is a low level client that is
/// useful for building higher level native clients or for building clients in other langauges via
/// FFI.
pub struct HaretClient {
    pub client_id: String,
    pub api_addr: Option<String>,
    pub namespace_id: Option<String>,
    pub primary: Option<ApiPid>,
    sock: Option<TcpStream>,
    request_num: u64
}

impl HaretClient {
    pub fn new(client_id: String) -> HaretClient {
        HaretClient {
            client_id: client_id,
            api_addr: None,
            namespace_id: None,
            primary: None,
            sock: None,
            request_num: 0
        }
    }

    /// Connect to `self.api_addr`
    pub fn connect(&mut self, api_addr: Option<String>) -> Result<()> {
        if api_addr.is_none() && self.api_addr.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput,
                              "API Address unknown. Please call connect with an api_addr."));
        }
        if api_addr.is_some() {
            self.api_addr = api_addr;
        }
        self.sock = Some(TcpStream::connect(&self.api_addr.as_ref().unwrap()[..])?);
        Ok(())
    }

    /// Register the client id on this node for the given namespace.
    ///
    /// This function returns the registration message to be written or an error if the primary is
    /// unknown.
    pub fn register(&mut self, primary: Option<ApiPid>) -> Result<String> {
        let request = self.prepare_register(primary)?;
        self.exec(request)
    }

    /// Register the client id on this node for the given namespace.
    ///
    /// This function returns the registration message to be written or an error if the primary is
    /// unknown.
    fn prepare_register(&mut self, primary: Option<ApiPid>) -> Result<ApiRequest> {
        if primary.is_none() && self.primary.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Primary unknown"));
        }

        if primary.is_some() {
            self.primary = primary;
            self.namespace_id = Some(self.primary.as_ref().unwrap()
                                     .get_group().to_string());
        }
        let namespace_id = self.namespace_id.clone();
        let mut msg = RegisterClient::new();
        msg.set_client_id(self.client_id.clone());
        msg.set_namespace_id(namespace_id.as_ref().unwrap().clone());
        let mut request = ApiRequest::new();
        request.set_register_client(msg);
        Ok(request)
    }

    pub fn list_namespaces(&mut self) -> Result<String> {
        let mut request = ApiRequest::new();
        request.set_get_namespaces(true);
        self.exec(request)
    }

    pub fn enter_namespace<T>(&mut self, namespace_id: T) -> Result<String>
        where T: Into<String>,
    {
        self.reset_primary();
        let mut msg = RegisterClient::new();
        msg.set_client_id(self.client_id.clone());
        msg.set_namespace_id(namespace_id.into());
        let mut request = ApiRequest::new();
        request.set_register_client(msg);
        self.exec(request)
    }

    pub fn ls(&mut self) -> Result<String> {
        let mut list_keys = ListKeys::new();
        list_keys.set_path("/".to_string());

        let mut tree_op = TreeOp::new();
        tree_op.set_list_keys(list_keys);

        let mut consensus_req = ConsensusRequest::new();
        consensus_req.set_to(self.primary.as_ref().unwrap().clone());
        consensus_req.set_client_id(self.client_id.clone());
        consensus_req.set_client_request_num(self.request_num);
        consensus_req.set_tree_op(tree_op);

        let mut request = ApiRequest::new();
        request.set_consensus_request(consensus_req);
        self.exec(request)
    }

    pub fn create(&mut self, path: &str, str_type: &str) -> Result<String> {
        let node_type = match str_type {
            "blob" => NodeType::BLOB,
            "queue" => NodeType::QUEUE,
            "set" => NodeType::SET,
            _ => unreachable!()
        };
        let mut create_node = CreateNode::new();
        create_node.set_path(path.to_string());
        create_node.set_node_type(node_type);
        let mut tree_op = TreeOp::new();
        tree_op.set_create_node(create_node);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn blob_put(&mut self, blob: &str, path: &str) -> Result<String> {
        let mut blob_put = BlobPut::new();
        blob_put.set_path(path.to_string());
        blob_put.set_val(blob.as_bytes().to_vec());
        let mut tree_op = TreeOp::new();
        tree_op.set_blob_put(blob_put);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn blob_get(&mut self, path: &str) -> Result<String> {
        let mut blob_get = BlobGet::new();
        blob_get.set_path(path.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_blob_get(blob_get);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn blob_size(&mut self, path: &str) -> Result<String> {
        let mut blob_size = BlobSize::new();
        blob_size.set_path(path.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_blob_size(blob_size);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn queue_push(&mut self, blob: &str, path: &str) -> Result<String> {
        let mut queue_push = QueuePush::new();
        queue_push.set_path(path.to_string());
        queue_push.set_val(blob.as_bytes().to_vec());
        let mut tree_op = TreeOp::new();
        tree_op.set_queue_push(queue_push);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn queue_pop(&mut self, path: &str) -> Result<String> {
        let mut queue_pop = QueuePop::new();
        queue_pop.set_path(path.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_queue_pop(queue_pop);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn queue_front(&mut self, path: &str) -> Result<String> {
        let mut queue_front = QueueFront::new();
        queue_front.set_path(path.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_queue_front(queue_front);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn queue_back(&mut self, path: &str) -> Result<String> {
        let mut queue_back = QueueBack::new();
        queue_back.set_path(path.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_queue_back(queue_back);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn queue_len(&mut self, path: &str) -> Result<String> {
        let mut queue_len = QueueLen::new();
        queue_len.set_path(path.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_queue_len(queue_len);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn set_insert(&mut self, blob: &str, path: &str) -> Result<String> {
        let mut set_insert = SetInsert::new();
        set_insert.set_path(path.to_string());
        set_insert.set_val(blob.as_bytes().to_vec());
        let mut tree_op = TreeOp::new();
        tree_op.set_set_insert(set_insert);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn set_remove(&mut self, blob: &str, path: &str) -> Result<String> {
        let mut set_remove = SetRemove::new();
        set_remove.set_path(path.to_string());
        set_remove.set_val(blob.as_bytes().to_vec());
        let mut tree_op = TreeOp::new();
        tree_op.set_set_remove(set_remove);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn set_contains(&mut self, blob: &str, path: &str) -> Result<String> {
        let mut set_contains = SetContains::new();
        set_contains.set_path(path.to_string());
        set_contains.set_val(blob.as_bytes().to_vec());
        let mut tree_op = TreeOp::new();
        tree_op.set_set_contains(set_contains);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn set_union<I, T>(&mut self, paths: I) -> Result<String>
        where I: Iterator<Item=T>,
              T: Into<String>,
    {
        let paths = paths.map(|s| s.into()).collect();
        let mut set_union = SetUnion::new();
        set_union.set_paths(RepeatedField::from_vec(paths));
        let mut tree_op = TreeOp::new();
        tree_op.set_set_union(set_union);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    pub fn set_intersection(&mut self, path1: &str, path2: &str) -> Result<String> {
        let mut set_intersection = SetIntersection::new();
        set_intersection.set_path1(path1.to_string());
        set_intersection.set_path2(path2.to_string());
        let mut tree_op = TreeOp::new();
        tree_op.set_set_intersection(set_intersection);
        let request = self.consensus_request(tree_op);
        self.exec(request)
    }

    fn consensus_request(&mut self, tree_op: TreeOp) -> ApiRequest {
        let mut consensus_req = ConsensusRequest::new();
        consensus_req.set_to(self.primary.as_ref().unwrap().clone());
        consensus_req.set_client_id(self.client_id.clone());
        consensus_req.set_client_request_num(self.request_num);
        consensus_req.set_tree_op(tree_op);
        let mut api_request = ApiRequest::new();
        api_request.set_consensus_request(consensus_req);
        api_request
    }

    fn exec(&mut self, req: ApiRequest) -> Result<String> {
        self.write_msg(req).map_err(|_| {
            Error::new(ErrorKind::NotConnected,
                       "Failed to write to socket. Please restart client and try again".to_string())
        })?;
        let mut api_response = self.read_msg().map_err(|_| {
            Error::new(ErrorKind::NotConnected,
                       "Failed to read from socket. Please restart client and try again".to_string())
        })?;

        if api_response.has_consensus_reply() {
            let mut consensus_reply = api_response.take_consensus_reply();

            let mut s = String::new();

            if consensus_reply.has_ok() {
                s.push_str("Ok\n");
            }

            if consensus_reply.has_tree_op_result() {
               s.push_str(&tree_op_result_to_string(consensus_reply.take_tree_op_result()));
            }

            if consensus_reply.has_tree_cas_result() {
                for result in consensus_reply.take_tree_cas_result().take_results().into_iter() {
                    s.push_str(&tree_op_result_to_string(result));
                }
            }

            if consensus_reply.has_path() {
                s.push_str(&consensus_reply.take_path());
            }

            if consensus_reply.has_error() {
                s.push_str("Error: ");
                s.push_str(&api_error_to_string(consensus_reply.take_error()));
                s.push('\n');
            }

            s.push_str(&format!("Epoch = {}, View = {}, Client Request Num = {}",
                                consensus_reply.get_epoch(),
                                consensus_reply.get_view(),
                                consensus_reply.get_request_num()));
            return Ok(s);
        }

        if api_response.has_namespaces() {
            let namespaces = api_response.take_namespaces().take_ids().to_vec();
            return Ok(namespaces.iter().fold(String::new(), |mut acc, namespace_id | {
                    acc.push_str(namespace_id);
                    acc.push_str("\n");
                    acc
            }));
        }

        if api_response.has_client_registration() {
            self.primary = Some(api_response.take_client_registration().take_primary());
            return Ok(format!("Client registered. Primary = {:?}", self.primary.as_ref().unwrap()));
        }

        if api_response.has_redirect() {
            let mut redirect = api_response.take_redirect();
            let primary = redirect.take_primary();
            let api_addr = redirect.take_api_addr();
            self.connect(Some(api_addr))?;
            let req = self.prepare_register(Some(primary.clone()))?;
            /// Todo: Remove this recursion to prevent potential stack overflow
            self.exec(req)?;
            return Ok(format!("Finished Redirecting. Primary = {:?}, API Address = {}",
                       self.primary.as_ref().unwrap(),
                       self.api_addr.as_ref().unwrap()))
        }

        if api_response.has_retry() {
            let duration = api_response.take_retry().get_milliseconds();
            return Ok(format!("Primary not found. Please retry in {} seconds.", duration / 1000));
        }

        if api_response.has_unknown_namespace() {
            return Ok("Unknown namespace".to_string());
        }

        if api_response.has_timeout() {
            return Ok("Timeout".to_string());
        }

        Ok(format!("unknown message {:?}", api_response))
    }

    fn reset_primary(&mut self) {
        self.primary = None;
        self.namespace_id = None;
    }

    fn write_msg(&mut self, req: ApiRequest) -> Result<()> {
        let mut msg = ApiMsg::new();
        msg.set_request(req);
        let encoded = msg.write_to_bytes().map_err(|_| {
            Error::new(ErrorKind::InvalidInput, "Failed to encode msgpack data")
        })?;
        let len: u32 = encoded.len() as u32;
        // 4 byte len header
        let header: [u8; 4] = unsafe { mem::transmute(len.to_be()) };
        self.sock.as_ref().unwrap().write_all(&header)?;
        self.sock.as_ref().unwrap().write_all(&encoded)?;
        self.request_num += 1;
        Ok(())
    }

    fn read_msg(&mut self) -> Result<ApiResponse> {
        let mut header = [0; 4];
        self.sock.as_mut().unwrap().read_exact(&mut header)?;
        let len = unsafe { u32::from_be(mem::transmute(header)) };
        let mut buf = vec![0; len as usize];
        self.sock.as_mut().unwrap().read_exact(&mut buf)?;
        let mut msg: ApiMsg = parse_from_bytes(&buf[..]).map_err(|e| {
            Error::new(ErrorKind::InvalidData, e.to_string())
        })?;
        Ok(msg.take_response())
    }
}

fn tree_op_result_to_string(mut result: TreeOpResult) -> String {
    let mut s = String::new();

    if result.has_ok() {
        s.push_str("Ok\n");
    } else if result.has_empty() {
        s.push_str("\n");
    } else if result.has_bool() {
        s.push_str(&format!("{}\n", result.get_bool()));
    } else if result.has_blob() {
        s.push_str(&format_blob(result.take_blob()));
    } else if result.has_int() {
        s.push_str(&format!("{:?}\n", result.get_int()));
    } else if result.has_set() {
        s.push_str(&format_set(result.take_set()));
    } else if result.has_keys() {
        for mut key in result.take_keys().take_keys().into_vec() {
            s.push_str(&format!("{}\n", key.take_name()));
        }
    }

    if result.has_optional_version() {
        s.push_str(&format!("Version = {} ", result.get_optional_version()));
    }
    s
}

fn format_blob(blob: Vec<u8>) -> String {
    match String::from_utf8(blob) {
        Ok(s) => format!("{}\n", s),
        Err(e) => format!("{:?}\n", e.into_bytes())
    }
}

fn format_set(mut set: Set) -> String {
    set.take_val().into_vec().into_iter().fold(String::new(), |mut acc, blob| {
        acc.push_str(&format_blob(blob));
        acc
    })
}

fn api_error_to_string(mut error: ApiError) -> String {
    if error.has_not_found() {
        format!("Path Not found: {}", error.take_not_found().take_path())
    } else if error.has_already_exists() {
        format!("Path Already exists: {}", error.take_already_exists().take_path())
    } else if error.has_does_not_exist() {
        format!("Path Does not exist: {}", error.take_does_not_exist().take_path())
    } else if error.has_wrong_type() {
        "Wrong type".to_string()
    } else if error.has_path_must_end_in_directory() {
        format!("Path must end in directory: {}",
                error.take_path_must_end_in_directory().take_path())
    } else if error.has_path_must_be_absolute() {
        "Paths must be absolute".to_string()
    } else if error.has_cas_failed() {
        "Cas Failed".to_string()
    } else if error.has_bad_format() {
        format!("Path is malformatted {}", error.take_bad_format().take_msg())
    } else if error.has_io() {
        format!("IO error: {}", error.take_io().take_msg())
    } else if error.has_encoding() {
        format!("Encoding error: {}", error.take_encoding().take_msg())
    } else if error.has_invalid_cas() {
        format!("Invalid CAS: {}", error.take_invalid_cas().take_msg())
    } else if error.has_msg() {
        error.take_msg()
    } else if error.has_cannot_delete_root() {
        "Cannot delete root".to_string()
    } else if error.has_invalid_msg() {
        "Invalid Message".to_string()
    } else if error.has_timeout() {
        "Timeout".to_string()
    } else if error.has_not_enough_replicas() {
        "Not enough replicas".to_string()
    } else if error.has_bad_epoch() {
        "Bad epoch".to_string()
    } else {
        "Unknown Error".to_string()
    }
}
