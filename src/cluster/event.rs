use mio::Token;
use state::State;

pub enum ClusterEvent {
    // ipstr has format host:port
    Join {token: Token, ipstr: String},
    State(State)
}
