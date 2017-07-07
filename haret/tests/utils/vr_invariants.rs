// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module contains assertion functions around VR protocol invariants. It's general enough to
//! be used from multiple tests.

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

pub fn assert_global_min_accept(states: &[VrState]) -> Result<(), String> {
    for state in states {
        let ctx = state.ctx();
        safe_assert!(ctx.log_start <= ctx.global_min_accept)?;
        safe_assert!(ctx.log_start <= ctx.commit_num)?;
    }
    Ok(())
}
