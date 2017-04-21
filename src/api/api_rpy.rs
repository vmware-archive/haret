use std::convert::TryFrom;
use rabble::{Pid, Error, Result, ErrorKind};
use super::messages as pb_api;
use pb_msg;

type Milliseconds = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiRpy {
    ClientRegistration {primary: Pid, new_registration: bool},
    Redirect {primary: Pid, api_addr: String},
    Retry(Milliseconds),
    UnknownNamespace
}


// The implementation for the client API protobuf message
impl From<ApiRpy> for pb_api::ApiResponse {
    fn from(reply: ApiRpy) -> pb_api::ApiResponse {
        match reply {
            ApiRpy::ClientRegistration {primary, new_registration} => {
                let mut cr = pb_api::ClientRegistration::new();
                cr.set_primary(primary.into());
                cr.set_new_registration(new_registration);
                let mut response = pb_api::ApiResponse::new();
                response.set_client_registration(cr);
                response
            },
            ApiRpy::Redirect {primary, api_addr} => {
                let mut redirect = pb_api::Redirect::new();
                redirect.set_primary(primary.into());
                redirect.set_api_addr(api_addr);
                let mut response = pb_api::ApiResponse::new();
                response.set_redirect(redirect);
                response
            },
            ApiRpy::Retry(milliseconds) => {
                let mut retry = pb_api::Retry::new();
                retry.set_milliseconds(milliseconds);
                let mut response = pb_api::ApiResponse::new();
                response.set_retry(retry);
                response
            },
            ApiRpy::UnknownNamespace => {
                let mut response = pb_api::ApiResponse::new();
                response.set_unknown_namespace(true);
                response
            }
        }
    }
}

// The Pid structure is different in the API
// TODO: Change this (requires updating clients)
impl From<Pid> for pb_api::ApiPid {
    fn from(pid: Pid) -> pb_api::ApiPid {
        let mut api_pid = pb_api::ApiPid::new();
        api_pid.set_name(pid.name);
        api_pid.set_node_name(pid.node.name);
        api_pid.set_node_addr(pid.node.addr);
        if pid.group.is_some() {
            api_pid.set_group(pid.group.unwrap());
        }
        api_pid
    }
}

// The implementation for the Internode messaging API
impl From<ApiRpy> for pb_msg::ApiRpy {
    fn from(reply: ApiRpy) -> pb_msg::ApiRpy {
        match reply {
            ApiRpy::ClientRegistration {primary, new_registration} => {
                let mut cr = pb_msg::ClientRegistration::new();
                cr.set_primary(primary.into());
                cr.set_new_registration(new_registration);
                let mut response = pb_msg::ApiRpy::new();
                response.set_client_registration(cr);
                response
            },
            ApiRpy::Redirect {primary, api_addr} => {
                let mut redirect = pb_msg::Redirect::new();
                redirect.set_primary(primary.into());
                redirect.set_api_addr(api_addr);
                let mut response = pb_msg::ApiRpy::new();
                response.set_redirect(redirect);
                response
            },
            ApiRpy::Retry(milliseconds) => {
                let mut response = pb_msg::ApiRpy::new();
                response.set_retry(milliseconds);
                response
            },
            ApiRpy::UnknownNamespace => {
                let mut response = pb_msg::ApiRpy::new();
                response.set_unknown_namespace(true);
                response
            }
        }
    }
}

impl TryFrom<pb_msg::ApiRpy> for ApiRpy {
    type Error = Error;
    fn try_from(msg: pb_msg::ApiRpy) -> Result<ApiRpy> {
        if msg.has_client_registration() {
            let reg = msg.take_client_registration();
            return Ok(ApiRpy::ClientRegistration {
                primary: reg.take_primary().into(),
                new_registration: reg.get_new_registration()
            });
        }
        if msg.has_redirect() {
            let redirect = msg.take_redirect();
            return Ok(ApiRpy::Redirect {
                primary: redirect.take_primary().into(),
                api_addr: redirect.take_api_addr()
            });
        }
        if msg.has_retry() {
            return Ok(ApiRpy::Retry(msg.take_retry()));
        }
        if msg.has_unknown_namespace() {
            return Ok(ApiRpy::UnknownNamespace);
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown ApiRpy").into())
    }
}
