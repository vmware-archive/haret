use config::Config;
use std::sync::{Arc, RwLock, Mutex};
use membership::Members;
use std::sync::mpsc::{Sender};
use cluster;
use admin;
use cluster::ClusterEvent;
use admin::AdminEvent;
use event::Event;
use mio::Token;

pub struct State {
    pub config: Arc<RwLock<Config>>,
    pub members: Members,
    pub cluster_tx: Option<Sender<Event<ClusterEvent, cluster::Msg>>>,
    pub admin_tx: Option<Sender<Event<AdminEvent, admin::Msg>>>,
    token: Arc<Mutex<Token>>

}

impl State {
    pub fn new() -> State {
        let config = Config::read();
        let members = Members::new(config.node_name.clone(),
                                   config.cluster_name.clone(),
                                   config.cluster_host.clone());
        State {
            config: Arc::new(RwLock::new(config)),
            members: members,
            cluster_tx: None,
            admin_tx: None,
            token: Arc::new(Mutex::new(Token(0)))
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = self.token.lock().unwrap();
        let Token(count) = *token;
        *token = Token(count + 1);
        *token
    }

}

impl Clone for State {
    fn clone(&self) -> State {
        let cluster_tx = match self.cluster_tx {
            Some(ref tx) => Some(tx.clone()),
            None => None
        };
        let admin_tx = match self.admin_tx {
            Some(ref tx) => Some(tx.clone()),
            None => None
        };
        State {
            config: self.config.clone(),
            members: self.members.clone(),
            cluster_tx: cluster_tx,
            admin_tx: admin_tx,
            token: self.token.clone()
        }
    }
}
