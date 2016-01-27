use config::Config;
use std::sync::{Arc, RwLock, Mutex};
use membership::Members;
use mio::Token;

pub struct State {
    pub config: Arc<RwLock<Config>>,
    pub members: Members,
    token: Arc<Mutex<Token>>

}

impl State {
    pub fn new() -> State {
        let config = Config::read();
        let members = Members::new(config.node_name.clone(),
                                   config.cluster_host.clone(),
                                   config.vr_host.clone(),
                                   config.vr_api_host.clone());
        State {
            config: Arc::new(RwLock::new(config)),
            members: members,
            token: Arc::new(Mutex::new(Token(0)))
        }
    }

    // This would be way more efficient with an atomic u64 instead of a lock
    pub fn next_token(&mut self) -> Token {
        let mut token = self.token.lock().unwrap();
        let Token(count) = *token;
        *token = Token(count + 1);
        *token
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        State {
            config: self.config.clone(),
            members: self.members.clone(),
            token: self.token.clone()
        }
    }
}
