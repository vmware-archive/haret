extern crate v2r2;
extern crate rabble;

use std::sync::mpsc::channel;
use v2r2::config::Config;
use v2r2::admin::{AdminReq, AdminServer};
use v2r2::vr::VrMsg;

fn main() {

    let config = Config::read();
    let node_id = NodeId {
        name: config.node_name,
        addr: config.cluster_host
    };

    let (node, handles) = rabble::rouse::<VrMsg>(node_id, None);



    let dispatcher = Dispatcher::new(&state);
    let dispatcher_tx = dispatcher.admin_tx.clone();
    let handles1 = dispatcher.run();
    let admin_server = AdminServer::new(state.clone(), dispatcher_tx.clone(), cluster_tx);
    let handles3 = admin_server.run();

    for l in vec![handles1, handles2, handles3] {
        for h in l {
            h.join().unwrap();
        }
    }
}
