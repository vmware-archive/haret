// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module contains assertion functions around VR protocol invariants. It's general enough to
//! be used from multiple tests.

use std::u64::MAX;
use haret::vr::VrState;

pub fn assert_single_primary_per_epoch_view(states: &[VrState]) -> Result<(), String> {
    // List of epoch/views for all primaries
    let mut epoch_view = None;
    for state in states {
        if let VrState::Primary(_) = *state {
            match epoch_view {
                None => epoch_view = Some((state.ctx().epoch, state.ctx().view)),
                Some((epoch, view)) => {
                    return safe_assert!(epoch != state.ctx().epoch || view != state.ctx().view)
                }
            }
        }
    }
    Ok(())
}

pub fn assert_minority_of_nodes_recovering(quorum: usize,
                                           states: &[VrState]) -> Result<(), String>
{
    let mut recovering_count = 0;
    for state in states {
        if let VrState::Recovery(_) = *state {
            recovering_count += 1;
        }
    }
    safe_assert!(recovering_count < quorum)
}

pub fn assert_quorum_of_logs_equal_up_to_smallest_commit(quorum: usize,
                                                         states: &[VrState])
    -> Result<(), String>
{
    let mut smallest_commit: u64 = MAX;
    for state in states {
        if state.ctx().commit_num < smallest_commit {
            smallest_commit = state.ctx().commit_num;
        }
    }
    if smallest_commit == 0 { return Ok(()) }

    let mut slice = None;
    let mut count = 0;
    for state in states {
        let ctx = state.ctx();
        if ctx.commit_num >= smallest_commit {
            match slice {
                None => {
                    // We define the log prefix we will check in the next iteration
                    slice = Some(&ctx.log[0..smallest_commit as usize]);
                    count += 1;
                },
                Some(s) => {
                    // Are the log prefixes the same?
                    safe_assert_eq!(s, &ctx.log[0..smallest_commit as usize])?;
                    count += 1;
                }
            }
        }
    }
    safe_assert!(count >= quorum)
}

pub fn assert_global_min_accept(states: &[VrState]) -> Result<(), String> {
    for state in states {
        let ctx = state.ctx();
        safe_assert!(ctx.commit_num >= ctx.global_min_accept)?;
        safe_assert!(ctx.commit_num >= ctx.global_min_accept)?;
    }
    Ok(())
}
