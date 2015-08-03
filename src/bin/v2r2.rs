extern crate v2r2;

use v2r2::state::State;
use v2r2::admin::AdminServer;
use v2r2::cluster::ClusterServer;
use v2r2::event_loop;

fn main() {
    let state = State::new();
    let (state, handle1)  = event_loop::run::<ClusterServer>(state.clone());
    let (state, handle2) = event_loop::run::<AdminServer>(state);
    handle1.join().unwrap();
    handle2.join().unwrap();
}
