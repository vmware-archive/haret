use std::io::Result;
use mio::Token;

#[derive(Debug)]
pub enum AdminMsg {
    // Msg to send to the cluster handler to initiate a join
    Join {token: Token, ipstr: String},
    // Response sent by cluster handler in response to a cluster join
    JoinReply {token: Token, reply: Result<()>}
}
