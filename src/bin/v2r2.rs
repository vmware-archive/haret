extern crate v2r2;

use std::sync::mpsc::channel;
use v2r2::state::State;
use v2r2::admin;
use v2r2::admin::AdminMsg;
use v2r2::cluster;

fn main() {
    let state = State::new();

    // The cluster server needs to be able to send and receive Admin Messages
    let (cluster_tx, admin_rx) = channel::<AdminMsg>();
    let (admin_tx, cluster_rx) = channel::<AdminMsg>();

    let handles1 = cluster::server::run(state.clone(), cluster_tx, cluster_rx);
    let handles2 = admin::server::run(state, admin_tx, admin_rx);

    for h in handles1 {
        h.join().unwrap();
    }
    for h in handles2 {
        h.join().unwrap();
    }
}
