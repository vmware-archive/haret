use rabble::{Pid, Envelope, CorrelationId};
use msg::Msg;
use vr::vr_fsm::{State, VrState};
use vr::vr_msg::{self, StartView, GetState};
use vr::states::{Primary, Backup, StartViewChange, DoViewChange, StateTransfer, Reconfiguration};
use vr::states::{Leaving, Recovery};

pub fn handle_start_view_change<T: State>(state: T,
                                          msg: vr_msg::StartViewChange,
                                          from: Pid,
                                          output: &mut Vec<Envelope<Msg>>) -> VrState
{
    // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
    // since we know we are out of date and need to perform state transfer, which will fail until
    // a replica is in normal mode.
    if msg.epoch != state.borrow_ctx().epoch {
        return state.into();
    }
    if msg.view <= state.borrow_ctx().view {
        return state.into();
    }

    StartViewChange::start_view_change(state.ctx(), from, msg, output)
}

pub fn handle_do_view_change<T: State>(state: T,
                                       msg: vr_msg::DoViewChange,
                                       from: Pid,
                                       output: &mut Vec<Envelope<Msg>>) -> VrState
{
    // Old messages we want to ignore. We don't want to become the primary here either, since we
    // didn't participate in reconfiguration, and therefore haven't yet learned about how many
    // replicas we need to get quorum. We just want to wait until another replica is elected
    // primary and then transfer state from it.
    if msg.epoch != state.borrow_ctx().epoch {
        return state.into();
    }
    if msg.view <= state.borrow_ctx().view {
        return state.into();
    }
    DoViewChange::start_do_view_change(state, from, msg, output)
}

pub fn handle_start_view<T: State>(state: T,
                                   msg: StartView,
                                   output: &mut Vec<Envelope<Msg>>) -> VrState
{
    if msg.epoch < state.borrow_ctx().epoch {
        return state.into();
    }
    if msg.epoch == state.borrow_ctx().epoch && msg.view <= state.borrow_ctx().view {
        return state.into();
    }
    // A primary has been elected in a new view / epoch
    // Even if the epoch is larger here, we will learn it and the new config by playing the log
    let StartView {view, op, log, commit_num, ..} = msg;
    Backup::become_backup(state.ctx(), view, op, log, commit_num, output)
}

pub fn handle_get_state<T: State>(state: T,
                                  msg: GetState,
                                  from: Pid,
                                  cid: CorrelationId,
                                  output: &mut Vec<Envelope<Msg>>) -> VrState
{
    let GetState {epoch, view, op} = msg;
    if epoch != state.borrow_ctx().epoch || view != state.borrow_ctx().view {
        return state.into()
    }
    StateTransfer::send_new_state(state.borrow_ctx(), op, from, cid, output);
    state.into()
}

pub fn handle_recovery<T: State>(state: T,
                                 msg: vr_msg::Recovery,
                                 from: Pid,
                                 cid: CorrelationId,
                                 output: &mut Vec<Envelope<Msg>>) -> VrState
{

    Recovery::send_response(state.borrow_ctx(), from, msg, cid, output);
    state.into()
}

pub fn handle_start_epoch<T: State>(state: T,
                                    from: Pid,
                                    cid: CorrelationId,
                                    output: &mut Vec<Envelope<Msg>>) -> VrState
{
    Reconfiguration::send_epoch_started(state.borrow_ctx(), from, cid, output);
    state.into()
}

/// The primary or backup has just committed the reconfiguration request. It must now determine
/// whether it is the primary of view 0 in the new epoch, a backup in the new epoch, or it is being
/// shutdown.
pub fn enter_transitioning<T: State>(state: T, output: &mut Vec<Envelope<Msg>>) -> VrState {
    if state.borrow_ctx().is_leaving() {
        return Leaving::leave(state.ctx());
    }
    // Tell replicas that are being replaced to shutdown
    Reconfiguration::broadcast_epoch_started(state.borrow_ctx(), output);
    if state.borrow_ctx().is_primary() {
        return Primary::enter(state.ctx());
    }
    Backup::enter(state.ctx())
}


