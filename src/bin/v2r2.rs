extern crate v2r2;

use std::sync::mpsc::channel;
use v2r2::state::State;
use v2r2::admin::{AdminReq, AdminServer};
use v2r2::cluster::ClusterServer;
use v2r2::vr::Dispatcher;

fn main() {
    let state = State::new();

    // The cluster server needs to be able to receive admin requests
    let (cluster_tx, cluster_rx) = channel::<AdminReq>();

    let dispatcher = Dispatcher::new(&state);
    let dispatch_tx = dispatcher.dispatch_tx.clone();
    let handles1 = dispatcher.run();
    let cluster_server = ClusterServer::new(state.clone());
    let handles2 = cluster_server.run(cluster_rx);
    let admin_server = AdminServer::new(state.clone(), dispatch_tx.clone(), cluster_tx);
    let handles3 = admin_server.run();

    for l in vec![handles1, handles2, handles3] {
        for h in l {
            h.join().unwrap();
        }
    }
}
