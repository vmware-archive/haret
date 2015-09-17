extern crate v2r2;

use std::sync::mpsc::channel;
use v2r2::state::State;
use v2r2::admin;
use v2r2::admin::AdminMsg;
use v2r2::cluster;
use v2r2::vr_api;

fn main() {
    let state = State::new();

    // The cluster server needs to be able to send and receive Admin Messages
    let (cluster_tx, admin_rx) = channel::<AdminMsg>();
    let (admin_tx, cluster_rx) = channel::<AdminMsg>();

    let handles1 = cluster::server::run(state.clone(), cluster_tx, cluster_rx);
    let handles2 = admin::server::run(state.clone(), admin_tx, admin_rx);
    let handles3 = vr_api::server::run(state);

    for l in vec![handles1, handles2, handles3] {
        for h in l {
            h.join().unwrap();
        }
    }
}
