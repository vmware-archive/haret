use std::io::Result;
use mio::Token;

#[derive(Debug)]
pub enum AdminEvent {
    // Response sent by cluster server in response to a cluster join
    JoinReply {token: Token, reply: Result<()>}
}
