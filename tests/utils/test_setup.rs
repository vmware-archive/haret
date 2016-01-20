//! This module contains common test setup functions

use v2r2::vr::{Dispatcher, Replica, RawReplica, VrMsg};
use v2r2::Member;
use v2r2::state::State;

pub fn init_tenant() -> (Dispatcher, Vec<Replica>) {
    let state = State::new();
    let node = state.members.me.clone();

    let mut dispatcher = Dispatcher::new(&state);
    // Set the timeouts to zero to preclude the need for sleeps
    dispatcher.set_idle_timeout_ms(0);

    let raw_replicas = vec![RawReplica {name: "r1".to_string(), node: node.clone()},
                            RawReplica {name: "r2".to_string(), node: node.clone()},
                            RawReplica {name: "r3".to_string(), node: node.clone()}];

    let tenant = dispatcher.create_test_tenant(raw_replicas.clone());
    (dispatcher, raw_replicas.iter().cloned().map(|r| Replica::new(tenant, r)).collect())
}

pub fn elect_initial_leader(dispatcher: &mut Dispatcher, replicas: &Vec<Replica>) {
    // Force an inactivity timeout on a single replica.
    dispatcher.send(&replicas[0], VrMsg::Tick);

    // Let all state transition messages get sent. Note that this won't infinitely block because we
    // control ticks from the test process and don't automatically generate messages to send to
    // replicas.
    dispatcher.dispatch_all_received_messages();

    // Show that there is a single primary and two backups. Also show that for view 1, the primary
    // is the second replica (r2). This is because we round robin the replicas using view numbers as
    // described in the papper, and view 0 is the first replica, although there is never an actual
    //  primary in view 0.
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[0]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 1);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[1]) {
        assert_eq!(state, "primary");
        assert_eq!(ctx.view, 1);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[2]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 1);
    } else {
        assert!(false)
    }
}
