use std::collections::{HashMap, BTreeMap, HashSet};
use std::sync::mpsc::Sender;
use std::fmt;
use std::iter::FromIterator;
use uuid::Uuid;
use fsm::{FsmHandler, StateFn};
use super::replica::{Replica, VersionedReplicas};
use super::messages::*;
use super::dispatcher::DispatchMsg;
use vr_api::{VrBackend, VrApiReq, VrApiRsp};
use time::{SteadyTime, Duration};
use rand::{thread_rng, Rng};

pub const DEFAULT_IDLE_TIMEOUT_MS: i64 = 300;
pub const DEFAULT_PRIMARY_TICK_MS: i64 = 75;

#[derive(Debug, Clone)]
pub enum StartupState {
    InitialConfig,
    Recovery,
    Reconfiguration
}

/// The internal state of the VR FSM. Note that fields are only made public for visibility,
/// debugging and testing purposes.
#[derive(Debug, Clone)]
pub struct VrCtx {
    pub me: Replica,
    pub primary: Option<Replica>,
    pub epoch: u64,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub startup_state: StartupState,
    pub last_received_time: SteadyTime,
    pub last_normal_view: u64,

    /// The number of replicas needed to provide quorum
    pub quorum: u64,

    /// In many parts of the protocol a replica will be waiting for a quorum of responses. Each
    /// received message of the correct type is placed in `quorum_messages`. When quorum is complete
    /// or the state changes to no longer needing that particular quorum, quorum_messages should be
    /// cleared.
    pub quorum_messages: HashMap<Replica, VrMsg>,

    /// The primary uses `commit_quorum` to track the highest seen op number returned in PrepareOk
    /// from each replica. When it sees a quorum of values >= some op number it commits all ops up
    /// to the given number.
    pub commit_quorum: HashMap<Replica, u64>,

    pub log: Vec<VrMsg>,
    pub backend: VrBackend,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas,
    pub client_table: HashMap<Uuid, (u64, Option<VrMsg>)>,

    /// The nonce of the last sent recovery message
    pub recovery_nonce: Option<Uuid>,

    /// During recovery we must wait for a at least a quorum of replies and a primary from the latest
    /// view learned about in the quorum. We maintain track of the latest view primary heard from in
    /// `recovery_primary`
    pub recovery_primary: Option<(Replica, u64)>,

    /// Backups wait `idle_timeout_ms` between messages from the primary before initiating a view
    /// change.
    pub idle_timeout_ms: i64,

    /// If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    /// message to the backups. `idle_timeout_ms` should be at least twice as large as this value.
    pub primary_tick_ms: i64,

    sender: DebugSender<Envelope>,
    client_reply_sender: DebugSender<ClientReplyEnvelope>,
    dispatch_sender: DebugSender<DispatchMsg>
}

// Sender<T> is not Debug. Implement a wrapper where we can impl Debug so that we don't have to do
// it for the entire VrCtx.
#[derive(Clone)]
struct DebugSender<T> {
    sender: Sender<T>
}

impl<T> DebugSender<T> {
    pub fn new(sender: Sender<T>) -> DebugSender<T> {
        DebugSender {
            sender: sender
        }
    }

    pub fn send(&self, msg: T) {
        self.sender.send(msg).unwrap();
    }
}

impl<T> fmt::Debug for DebugSender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sender<T>")
    }
}

impl VrCtx {
    pub fn new(me: Replica,
               old_config: VersionedReplicas,
               new_config: VersionedReplicas,
               startup_state: StartupState,
               sender: Sender<Envelope>,
               client_reply_sender: Sender<ClientReplyEnvelope>,
               dispatch_sender: Sender<DispatchMsg>) -> VrCtx {
        VrCtx {
            me: me,
            primary: None,
            epoch: new_config.epoch,
            view: 0,
            op: 0,
            commit_num: 0,
            startup_state: startup_state,
            last_received_time: SteadyTime::now(),
            last_normal_view: 0,
            quorum: (new_config.replicas.len() / 2 + 1) as u64,
            quorum_messages: HashMap::new(),
            commit_quorum: HashMap::new(),
            log: Vec::new(),
            backend: VrBackend::new(),
            old_config: old_config,
            new_config: new_config,
            client_table: HashMap::new(),
            recovery_nonce: None,
            recovery_primary: None,
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS,
            sender: DebugSender::new(sender),
            client_reply_sender: DebugSender::new(client_reply_sender),
            dispatch_sender: DebugSender::new(dispatch_sender)
        }
    }
}

pub struct VrHandler;

impl FsmHandler for VrHandler {
    type Context = VrCtx;
    type Msg = VrMsg;

    fn initial_state() -> StateFn<VrHandler> {
        next!(startup)
    }
}

macro_rules! check_epoch {
    ($ctx:ident, $msg:ident, $state:ident) => {
        match $msg {
            VrMsg::StartViewChange {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::StartViewChange {epoch, ..} if epoch > $ctx.epoch => {
                return learn_config($ctx, epoch)
            },
            VrMsg::DoViewChange {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::DoViewChange {epoch, ..} if epoch > $ctx.epoch => {
                return learn_config($ctx, epoch)
            },
            VrMsg::StartView {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::StartView {epoch, ..} if epoch > $ctx.epoch => {
                return learn_config($ctx, epoch)
            },
            VrMsg::Prepare {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::Prepare {epoch, ..} if epoch > $ctx.epoch => {
                return learn_config($ctx, epoch)
            },
            VrMsg::Commit {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            // This is normal if a reconfiguration has just been committed and we are in the new
            // group and old group.
            VrMsg::Commit {epoch, commit_num, ..} if epoch == $ctx.epoch + 1 => {
                return handle_commit_new_epoch($ctx, commit_num, epoch)
            },
            // There have been multiple reconfigurations. We need to learn the latest config
            VrMsg::Commit {epoch, ..} if epoch > $ctx.epoch + 1 => {
                return learn_config($ctx, epoch)
            },
            VrMsg::GetState {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::GetState {epoch, ..} if epoch > $ctx.epoch => {
                return learn_config($ctx, epoch)
            },
            VrMsg::NewState {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::NewState {epoch, ..} if epoch > $ctx.epoch => {
                // This is a bug. NewState should only respond to current GetState requests
                assert!(false)
            },
            VrMsg::StartEpoch {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::StartEpoch {epoch, op, old_config, new_config} => {
                return handle_start_epoch($ctx, epoch, op, old_config, new_config)
            },
            // This would be quite strange, since these messages should only be sent to replicas
            // shutting down.
            VrMsg::EpochStarted {epoch, ..} if epoch < $ctx.epoch => return next!($state),
            VrMsg::EpochStarted {epoch, ..} if epoch > $ctx.epoch => {
                let stop = DispatchMsg::Stop($ctx.me.clone());
                $ctx.dispatch_sender.send(stop);
                return next!(shutdown)
            },
            _ => ()
        }
    }
}

/// Perform an early return to the correct state if the view is not current. We also handle
/// StartView and DoViewChange messages in here for all views, because it is common code across
/// primary and backups. Some messages are not handled in all states (like PrepareOk), so we don't
/// check them here.
macro_rules! check_view {
    ($ctx:ident, $msg:ident, $state:ident) => {
        match $msg {
            VrMsg::StartViewChange {view, ref from, ..} if view > $ctx.view => {
                reset_quorum($ctx, $msg.clone(), view, from.clone());
                return start_view_change($ctx)
            },
            VrMsg::StartViewChange {view, ..} if view < $ctx.view => return next!($state),
            VrMsg::DoViewChange {view, ref from, ..} if view >= $ctx.view => {
                reset_quorum($ctx, $msg.clone(), view, from.clone());
                return maybe_start_view($ctx, $msg.clone(), from.clone());
            },
            VrMsg::DoViewChange {view, ..} if view < $ctx.view => return next!($state),
            VrMsg::StartView {view, ..} if view >= $ctx.view => {
                return become_backup($ctx, $msg.clone())
            },
            VrMsg::StartView {view, ..} if view < $ctx.view => return next!($state),
            VrMsg::Prepare {view, ..} if view > $ctx.view => {
                return start_state_transfer_new_view($ctx, view)
            },
            VrMsg::Prepare {view, ..} if view < $ctx.view => return next!($state),
            VrMsg::Commit {view, ..} if view > $ctx.view => {
                return start_state_transfer_new_view($ctx, view)
            },
            VrMsg::Commit {view, ..} if view < $ctx.view => return next!($state),
            VrMsg::GetState {view, ..} if view > $ctx.view => {
                return start_state_transfer_new_view($ctx, view)
            },
            VrMsg::GetState {view, ..} if view < $ctx.view => return next!($state),
            _ => ()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////

/// When a replica starts it enters this state. It will determine from here whether it is starting
/// fresh during a reconfiguration, restarting after failure, or starting as the initial
/// configuration of a new tenant. All three cases lead to different initial states.
pub fn startup(ctx: &mut VrCtx, _msg: VrMsg) -> StateFn<VrHandler> {
    match ctx.startup_state {
        StartupState::InitialConfig => {
            ctx.view += 1;
            start_view_change(ctx)
        },
        StartupState::Recovery => {
            start_recovery(ctx)
        },
        StartupState::Reconfiguration => {
            start_state_transfer_transitioning(ctx)
        }
    }
}

/// This replica is currently the primary replica operating normally
pub fn primary(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, primary);
    check_view!(ctx, msg, primary);
    let msg2 = msg.clone();
    match msg {
        VrMsg::ClientRequest {op, client_id, request_num} => {
            handle_client_request(ctx, msg2, op, client_id, request_num);
            next!(primary)
        },
        VrMsg::PrepareOk {view, op, ref from, ..} if view == ctx.view && op > ctx.commit_num => {
            maybe_commit(ctx, op, from.clone());
            next!(primary)
        },
        VrMsg::Tick => {
            if primary_idle_timeout(ctx) {
                broadcast_commit_msg(ctx);
            }
            next!(primary)
        },
        VrMsg::GetState{..} => {
            send_new_state(ctx, msg);
            next!(primary)
        },
        VrMsg::Recovery{..} => {
            send_primary_recovery_response(ctx, msg);
            next!(primary)
        },
        VrMsg::Reconfiguration{..} => handle_reconfig(ctx, msg),
        _ => next!(primary)
    }
}

/// This replica is currently a backup operating normally
pub fn backup(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, backup);
    check_view!(ctx, msg, backup);
    let msg2 = msg.clone();
    match msg {
        VrMsg::Prepare {op, ..} if op == ctx.op + 1 => {
            handle_normal_prepare(ctx, msg);
            next!(backup)
        },
        VrMsg::Prepare {op, ..} if op > ctx.op + 1 => {
            start_state_transfer_same_view(ctx)
        },
        VrMsg::Commit {commit_num, ..} => {
            handle_commit(ctx, commit_num)
        },
        VrMsg::Tick => {
            if idle_timeout(ctx) {
                reset_and_start_view_change(ctx)
            } else {
                next!(backup)
            }
        },
        VrMsg::GetState{..} => {
            send_new_state(ctx, msg2);
            next!(backup)
        },
        VrMsg::Recovery{..} => {
            maybe_send_recovery_response(ctx, msg)
        },
        _ => next!(backup)
    }
}

/// This is the state of the primary after it has sent a Prepare message containing the client
/// reconfiguration request.
pub fn transitioning_primary(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, transitioning_primary);
    check_view!(ctx, msg, transitioning_primary);
    match msg {
        VrMsg::PrepareOk {view, op, ref from, ..} if view == ctx.view && op == ctx.op => {
            // We aren't accepting client requests in this state, so we use the time between
            // prepare responses as idle time
            ctx.last_received_time = SteadyTime::now();
            maybe_commit_reconfig(ctx, op, from.clone())
        },
        VrMsg::Tick => {
            // If we haven't received quorum of PrepareOk in time then re-broadcast Prepare
            if idle_timeout(ctx) {
                rebroadcast_reconfig(ctx);
            }
            next!(transitioning_primary)
        }
        _ => next!(transitioning_primary)
    }
}

/// This is the state used during reconfiguration.
pub fn transitioning(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, transitioning);
    check_view!(ctx, msg, transitioning);
    match msg {
        VrMsg::NewState {op, ..} if op >= ctx.new_config.op => {
            set_state(ctx, msg);
            if replicas_to_replace(ctx).contains(&ctx.me) {
                ctx.quorum_messages = HashMap::new();
                return next!(transitioning_to_replace)
            }
            send_epoch_started(ctx);
            if ctx.me == compute_primary(ctx) {
                return next!(primary)
            }
            next!(backup)
        },
        _ => next!(transitioning)
    }
}

/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
pub fn transitioning_to_replace(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, transitioning_to_replace);
    match msg {
        VrMsg::EpochStarted {ref from, ..} => {
            ctx.quorum_messages.insert(from.clone(), msg.clone());
            if quorum(&ctx) {
                let stop = DispatchMsg::Stop(ctx.me.clone());
                ctx.dispatch_sender.send(stop);
                return next!(shutdown)
            }
            next!(transitioning_to_replace)
        },
        VrMsg::Tick => {
            if idle_timeout(ctx) {
                broadcast_start_epoch(ctx);
            }
            next!(transitioning_to_replace)
        }
        _ => next!(transitioning_to_replace)
    }
}

/// This replica has already told the dispatcher to shut it down. It just waits in this state and
/// doesn't respond to any messages from this point out.
pub fn shutdown(ctx: &mut VrCtx, _: VrMsg) -> StateFn<VrHandler> {
    next!(shutdown)
}

pub fn recovery(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, recovery);
    match msg {
        VrMsg::RecoveryResponse {view, ref op, ref nonce, ref from, ..} if Some(*nonce) == ctx.recovery_nonce =>
            maybe_recover(ctx, msg.clone(), view, op.is_some(), from.clone()),
        _ => next!(recovery)
    }
}

/// A replica is in this state during view change. The replica remains in this state until view
/// change is completed unless it is the proposed primary, which transitions to the `do_view_change`
/// state on receipt of a `DoViewChange` message.
pub fn view_change(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, view_change);
    check_view!(ctx, msg, view_change);
    let msg2 = msg.clone();
    match msg {
        VrMsg::StartViewChange{ref from, ..} => {
            ctx.quorum_messages.insert(from.clone(), msg2);
            maybe_send_do_view_change(ctx)
        },
        VrMsg::Tick => {
            // We haven't changed views yet. The new primary must be down. Try again.
            if idle_timeout(ctx) {
                reset_and_start_view_change(ctx)
            } else {
                next!(view_change)
            }
        },
        VrMsg::Recovery {ref from, ..} if *from == *ctx.primary.as_ref().unwrap() => {
            // We don't handle recovery messages in view_change state. However, if we receive one
            // from the primary for this view we know the election will never complete and trigger a
            // new view change immediately rather than waiting. It's possible this is an old
            // recovery message lost in the network, but an extra view change is still safe.  It's
            // impossible that the recovery message is old when using TCP as transport.
            reset_and_start_view_change(ctx)
        },
        VrMsg::Prepare {view, ..} => start_state_transfer_new_view(ctx, view),
        VrMsg::Commit {view, ..} => start_state_transfer_new_view(ctx, view),
        _ => next!(view_change)
    }
}

/// At least one replica has received a quorum of `StartViewChange` messages for a given view.
/// It has sent a `DoViewChange` message to the proposed primary for that view. In this state the
/// proposed primary is waiting for a quorum of `DoViewChange` messages so that it can become the
/// primary for that view.
pub fn do_view_change(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, do_view_change);
    check_view!(ctx, msg, do_view_change);
    let msg2 = msg.clone();
    match msg {
        VrMsg::DoViewChange{from, ..} => maybe_start_view(ctx, msg2, from),
        _ => next!(do_view_change)
    }
}

/// When a backup realizes it's behind it enters this state
pub fn state_transfer(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    check_epoch!(ctx, msg, state_transfer);
    check_view!(ctx, msg, state_transfer);
    match msg {
        VrMsg::NewState {view, ..} => {
            set_state(ctx, msg);
            next!(backup)
        },
        _ => next!(state_transfer)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// END OF FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////

fn reset_and_start_view_change(ctx: &mut VrCtx) -> StateFn<VrHandler> {
    ctx.view += 1;
    ctx.commit_quorum = HashMap::new();
    ctx.quorum_messages = HashMap::new();
    start_view_change(ctx)
}

fn learn_config(ctx: &mut VrCtx, epoch: u64) -> StateFn<VrHandler> {
    unimplemented!();
}

// TODO: This should be made into a general function that rebroadcasts the last operation. It can
// only be a reconfig or client request in the log.
fn rebroadcast_reconfig(ctx: &mut VrCtx) {
    let reconfig = ctx.log[(ctx.op - 1) as usize].clone();
    if let VrMsg::Reconfiguration {client_id, client_req_num, replicas, ..} = reconfig {
        let prepare = VrMsg::Prepare {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            client_id: client_id,
            client_request_num: client_req_num,
            client_op: VrApiReq::Null,
            commit_num: ctx.commit_num
        };
        broadcast(ctx, prepare);
    }
}

fn maybe_commit_reconfig(ctx: &mut VrCtx, op: u64, from: Replica) -> StateFn<VrHandler> {
    maybe_commit(ctx, op, from);
    if ctx.commit_num == op {
        // We committed the reconfiguration request
        ctx.epoch += 1;
        ctx.view = 0;
        broadcast_commit_msg(ctx);
        ctx.old_config = ctx.new_config.clone();
        // Get the reconfiguration from the log. This always succeeds.
        if let VrMsg::Reconfiguration {ref replicas, ..} = ctx.log[(op-1) as usize] {
            ctx.new_config = VersionedReplicas {epoch: ctx.epoch,
                                                op: ctx.op,
                                                replicas: replicas.clone()};
        }
        let reconfig = DispatchMsg::Reconfiguration {
            tenant: ctx.me.tenant.clone(),
            old_config: ctx.old_config.clone(),
            new_config: ctx.new_config.clone()
        };
        ctx.dispatch_sender.send(reconfig);
        // We know that we are up to date with respect to the new epoch at this point
        send_epoch_started(ctx);
        if ctx.me == compute_primary(ctx) {
            next!(primary)
        } else {
            next!(backup)
        }
    } else {
        next!(transitioning_primary)
    }
}

/// A backup will send a recovery response if it's not to the current primary. Otherwise it will
/// start a view change.
fn maybe_send_recovery_response(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    if let VrMsg::Recovery {from, nonce} = msg {
        if *ctx.primary.as_ref().unwrap() == from {
            return reset_and_start_view_change(ctx)
        }
        let response = VrMsg::RecoveryResponse {
            epoch: ctx.epoch,
            view: ctx.view,
            nonce: nonce,
            from: ctx.me.clone(),
            op: None,
            commit_num: None,
            log: None
        };
        ctx.sender.send(Envelope::new(from, response));
    }
    next!(backup)
}

fn send_primary_recovery_response(ctx: &mut VrCtx, msg: VrMsg) {
    if let VrMsg::Recovery {from, nonce} = msg {
        let response = VrMsg::RecoveryResponse {
            epoch: ctx.epoch,
            view: ctx.view,
            nonce: nonce,
            from: ctx.me.clone(),
            op:  Some(ctx.op),
            commit_num: Some(ctx.commit_num),
            log: Some(ctx.log.clone())
        };
        ctx.sender.send(Envelope::new(from, response));
    }
}

fn broadcast_start_epoch(ctx: &mut VrCtx) {
    let msg = VrMsg::StartEpoch {
        epoch: ctx.epoch,
        op: ctx.op,
        old_config: ctx.old_config.clone(),
        new_config: ctx.new_config.clone()
    };
    broadcast(ctx, msg);
}

fn send_epoch_started(ctx: &mut VrCtx) {
    let msg = VrMsg::EpochStarted {
        epoch: ctx.epoch,
        from: ctx.me.clone()
    };

    for r in replicas_to_replace(ctx) {
        ctx.sender.send(Envelope::new(r, msg.clone()));
    }
}

fn replicas_to_replace(ctx: &VrCtx) -> Vec<Replica> {
    let new_set = HashSet::<Replica>::from_iter(ctx.new_config.replicas.clone());
    let old_set = HashSet::<Replica>::from_iter(ctx.old_config.replicas.clone());
    old_set.difference(&new_set).cloned().collect()
}

fn send_new_state(ctx: &mut VrCtx, msg: VrMsg) {
    if let VrMsg::GetState {op, from, ..} = msg {
        let mut log_tail = Vec::new();
        for i in op..ctx.op {
            log_tail.push(ctx.log[i as usize].clone());
        }
        let new_state = VrMsg::NewState {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            commit_num: ctx.commit_num,
            log_tail: log_tail
        };
        ctx.sender.send(Envelope::new(from, new_state));
    }
}

fn set_state(ctx: &mut VrCtx, msg: VrMsg) {
    if let VrMsg::NewState {view, op, commit_num, log_tail, ..} = msg {
        ctx.view = view;
        ctx.op = op;
        for m in log_tail {
            ctx.log.push(m);
        }
        backup_commit_known_committed_ops(ctx, commit_num);
    }
}

fn handle_start_epoch(ctx: &mut VrCtx,
                      epoch: u64,
                      op: u64,
                      old_config: VersionedReplicas,
                      new_config: VersionedReplicas) -> StateFn<VrHandler> {
    // We appear to have the latest request in the log. Let's check to see if it's a
    // reconfiguration request from the prior epoch.
    if ctx.op == op {
        if let VrMsg::Reconfiguration {epoch: log_epoch, ..} = ctx.log[(op-1) as usize] {
            if log_epoch + 1 == epoch {
                ctx.last_received_time = SteadyTime::now();
                ctx.epoch = epoch;
                ctx.view = 0;
                ctx.old_config = old_config;
                ctx.new_config = new_config;
                backup_commit_known_committed_ops(ctx, op);
                send_epoch_started(ctx);
                if ctx.me == compute_primary(ctx) {
                    return next!(primary)
                } else {
                    return next!(backup)
                }
            }
        }
    }
    ctx.last_received_time = SteadyTime::now();
    ctx.epoch = epoch;
    ctx.view = 0;
    ctx.old_config = old_config;
    ctx.new_config = new_config;
    return start_state_transfer_transitioning(ctx)
}

/// The primary in the old epoch has sent a commit with the new epoch number to the backups in the
/// old config. The backups bump their epoch number, ensure they have the latest state, commit
/// operations up to the reconfig and transition to either backup, primary, or
/// transitioning_to_replace if they are not a member of the new group.
fn handle_commit_new_epoch(ctx: &mut VrCtx, commit_num: u64, epoch: u64) -> StateFn<VrHandler> {
    ctx.last_received_time = SteadyTime::now();
    ctx.epoch = epoch;
    ctx.view = 0;
    let to_replace = replicas_to_replace(ctx);
    if commit_num == ctx.op {
        backup_commit_known_committed_ops(ctx, commit_num);
        if !to_replace.contains(&ctx.me) {
            send_epoch_started(ctx);
        }
    } else {
        return learn_config(ctx, epoch);
    }
    if to_replace.contains(&ctx.me) {
        ctx.quorum_messages = HashMap::new();
        return next!(transitioning_to_replace)
    }
    if ctx.me == compute_primary(ctx) {
        return next!(primary)
    } else {
        return next!(backup)
    }
}

fn handle_commit(ctx: &mut VrCtx, commit_num: u64) -> StateFn<VrHandler> {
    ctx.last_received_time = SteadyTime::now();
    // Note that a primary cannot have a commit_num smaller than a backup by protocol invariants
    if commit_num == ctx.commit_num { return next!(backup) }
    if commit_num == ctx.op {
        backup_commit_known_committed_ops(ctx, commit_num);
        return next!(backup)
    }
    // commit_num > ctx.op
    start_state_transfer_same_view(ctx)
}

fn start_state_transfer_transitioning(ctx: &mut VrCtx) -> StateFn<VrHandler> {
    ctx.op = ctx.commit_num;
    ctx.view = 0;
    ctx.log.truncate(ctx.op as usize);
    let msg = VrMsg::GetState {
        epoch: ctx.epoch,
        view: ctx.view,
        op: ctx.op,
        from: ctx.me.clone()
    };
    broadcast_old_and_new(ctx, msg);
    next!(transitioning)
}

fn start_state_transfer_same_view(ctx: &VrCtx) -> StateFn<VrHandler> {
    let msg = VrMsg::GetState {
        epoch: ctx.epoch,
        view: ctx.view,
        op: ctx.op,
        from: ctx.me.clone()
    };
    send_to_random_replica(ctx, msg, &ctx.new_config);
    next!(state_transfer)
}

fn start_state_transfer_new_view(ctx: &mut VrCtx, view: u64) -> StateFn<VrHandler> {
    ctx.op = ctx.commit_num;
    ctx.view = view;
    ctx.log.truncate(ctx.op as usize);
    start_state_transfer_same_view(ctx)
}

fn handle_normal_prepare(ctx: &mut VrCtx, msg: VrMsg) {
    if let VrMsg::Prepare { client_op, commit_num, client_id, client_request_num, ..} = msg {
        ctx.last_received_time = SteadyTime::now();
        ctx.op += 1;
        // Reconstruct Client Request, since the log stores VrMsgs
        let client_req = VrMsg::ClientRequest { op: client_op,
                                                client_id: client_id,
                                                request_num: client_request_num};
        ctx.log.push(client_req);
        ctx.client_table.insert(client_id, (client_request_num, None));
        let prepare_ok = VrMsg::PrepareOk { epoch: ctx.epoch,
                                            view: ctx.view,
                                            op: ctx.op,
                                            from: ctx.me.clone() };
        send_to_primary(ctx, prepare_ok);
        backup_commit_known_committed_ops(ctx, commit_num);
    }
}

fn send_to_random_replica(ctx: &VrCtx, msg: VrMsg, config: &VersionedReplicas) {
    let mut rng = thread_rng();
    let mut to = ctx.me.clone();
    while to == ctx.me {
        let index = rng.gen_range(0, config.replicas.len());
        to = config.replicas[index].clone()
    }
    ctx.sender.send(Envelope::new(to, msg));
}

fn send_to_primary(ctx: &VrCtx, msg: VrMsg) {
    ctx.sender.send(Envelope::new(ctx.primary.as_ref().unwrap().clone(), msg));
}

/// Handle a Reconfiguration request as the primary
fn handle_reconfig(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    if let VrMsg::Reconfiguration {client_id, client_req_num, epoch, ref replicas} = msg {
        if replicas.len() < 3 {
            let errmsg = "New config must contain at least 3 replicas".to_string();
            let reply = VrMsg::ClientReply {
                epoch: ctx.epoch,
                view: ctx.view,
                request_num: client_req_num,
                value: VrApiRsp::Error {msg: errmsg}
            };
            send_client_reply(ctx, client_id, reply);
            return next!(primary);
        }
        if epoch < ctx.epoch || epoch > ctx.epoch {
            let errmsg = "Reconfiguration attempted with incorrect epoch".to_string();
            let reply = VrMsg::ClientReply {
                epoch: ctx.epoch,
                view: ctx.view,
                request_num: client_req_num,
                value: VrApiRsp::Error {msg: errmsg}
            };
            send_client_reply(ctx, client_id, reply);
            return next!(primary);
        }
        handle_client_request(ctx, msg.clone(), VrApiReq::Null, client_id, client_req_num);
        return next!(transitioning_primary)
    }
    assert!(false);
    next!(transitioning_primary)
}

/// Handle a client request as the primary
fn handle_client_request(ctx: &mut VrCtx,
                         msg: VrMsg,
                         client_op: VrApiReq,
                         client_id: Uuid,
                         req_num: u64)
{
    if sent_last_response(ctx, &client_id, req_num) { return; }
    ctx.last_received_time = SteadyTime::now();
    ctx.client_table.insert(client_id, (req_num, None));
    ctx.op += 1;
    ctx.log.push(msg);
    let prepare = VrMsg::Prepare { epoch: ctx.epoch,
                                   view: ctx.view,
                                   op: ctx.op,
                                   client_id: client_id,
                                   client_request_num: req_num,
                                   client_op: client_op,
                                   commit_num: ctx.commit_num };
    broadcast(ctx, prepare);
}

fn sent_last_response(ctx: &mut VrCtx, client_id: &Uuid, req_num: u64) -> bool {
    if let Some(&(last_req_num, ref maybe_last_response)) = ctx.client_table.get(&client_id) {
        if last_req_num == req_num && maybe_last_response.is_some() {
            // Client replies must be sent with the latest epoch and view. This may be different
            // from when the operation was actually committed and stored in the client_table.
            let last_response = maybe_last_response.clone().unwrap();
            if let VrMsg::ClientReply {request_num, value, ..} = last_response {
                let reply = VrMsg::ClientReply {epoch: ctx.epoch,
                                                view: ctx.view,
                                                request_num: request_num,
                                                value: value};
                send_client_reply(ctx, client_id.clone(), reply);
                return true;
            }
        } else if last_req_num >= req_num {
            return true;
        } else {
            return false;
        }
    }
    return false;
}

fn send_client_reply(ctx: &VrCtx, client_id: Uuid, reply: VrMsg) {
    let envelope = ClientReplyEnvelope {to: client_id, msg: reply};
    ctx.client_reply_sender.send(envelope);
}

fn become_backup(ctx: &mut VrCtx, msg: VrMsg) -> StateFn<VrHandler> {
    if let VrMsg::StartView {view, op, log, commit_num, ..} = msg {
        ctx.view = view;
        ctx.op = op;
        ctx.log = log;
        ctx.commit_quorum = HashMap::new();
        ctx.quorum_messages = HashMap::new();
        send_prepare_ok_for_uncommitted_ops(ctx);
        backup_commit_known_committed_ops(ctx, commit_num);
        ctx.last_normal_view = ctx.view;
    }
    next!(backup)
}

fn backup_commit_known_committed_ops(ctx: &mut VrCtx, new_commit_num: u64) {
    for i in ctx.commit_num..new_commit_num {
        let msg = ctx.log[i as usize].clone();
        if let VrMsg::ClientRequest {op, client_id, request_num} = msg.clone() {
            let rsp = ctx.backend.call(i+1, op);
            let reply = VrMsg::ClientReply {epoch: ctx.epoch,
                                            view: ctx.view,
                                            request_num: request_num,
                                            value: rsp};
            ctx.client_table.insert(client_id, (request_num, Some(reply.clone())));
        }
        // We have already set the correct epoch in our context at this point
        if let VrMsg::Reconfiguration {client_id, client_req_num, replicas, ..} = msg {
            ctx.old_config = ctx.new_config.clone();
            ctx.new_config = VersionedReplicas {epoch: ctx.epoch, op: i+1, replicas: replicas};
            let rsp = ctx.backend.call(i+1, VrApiReq::Null);
            let reply = VrMsg::ClientReply {
                epoch: ctx.epoch,
                view: 0,
                request_num: client_req_num,
                value: rsp
            };
            ctx.client_table.insert(client_id, (client_req_num, Some(reply.clone())));
        }
    }
    ctx.commit_num = new_commit_num;
}

fn primary_commit_known_committed_ops(ctx: &mut VrCtx, last_commit_num: u64) {
    for i in last_commit_num..ctx.commit_num {
        let msg = ctx.log[i as usize].clone();
        if let VrMsg::ClientRequest {op, client_id, request_num} = msg.clone() {
            let rsp = ctx.backend.call(i+1, op.clone());
            let reply = VrMsg::ClientReply {epoch: ctx.epoch,
                                            view: ctx.view,
                                            request_num: request_num,
                                            value: rsp};
            ctx.client_table.insert(client_id.clone(), (request_num, Some(reply.clone())));
            send_client_reply(ctx, client_id, reply);
        }
        if let VrMsg::Reconfiguration {client_id, client_req_num, epoch, replicas} = msg {
            let rsp = ctx.backend.call(i+1, VrApiReq::Null);
            let reply = VrMsg::ClientReply {
                epoch: epoch,
                view: 0,
                request_num: client_req_num,
                value: rsp
            };
            ctx.client_table.insert(client_id.clone(), (client_req_num, Some(reply.clone())));
            send_client_reply(ctx, client_id, reply);
        }
    }
}

fn send_prepare_ok_for_uncommitted_ops(ctx: &VrCtx) {
    for i in ctx.commit_num..ctx.op {
        let msg = VrMsg::PrepareOk {epoch: ctx.epoch, view: ctx.view, op: i+1, from: ctx.me.clone()};
        send_to_primary(ctx, msg);
    }
}

fn reset_quorum(ctx: &mut VrCtx, msg: VrMsg, view: u64, from: Replica) {
    ctx.view = view;
    ctx.commit_quorum = HashMap::new();
    ctx.quorum_messages = HashMap::new();
    ctx.quorum_messages.insert(from, msg);
}

fn maybe_commit(ctx: &mut VrCtx, op: u64, from: Replica) {
    ctx.commit_quorum.insert(from.clone(), op);
    if let Some(n) = get_commit_quorum(&ctx) {
        let last_commit_num = ctx.commit_num;
        ctx.commit_num = n;
        primary_commit_known_committed_ops(ctx, last_commit_num);
    }
}

fn maybe_start_view(ctx: &mut VrCtx, msg: VrMsg, from: Replica) -> StateFn<VrHandler> {
    ctx.quorum_messages.insert(from, msg);
    if quorum(&ctx) {
        let last_commit_num = ctx.commit_num;
        set_latest_state(ctx);
        broadcast_start_view_msg(&ctx);
        // During a view change these ops are present, but an actual TCP client won't be connected.
        // It's likely they will need to be collected in buffers for resending on connection
        // from the clients to the new primary. Regardless, that's outside the scope of this FSM,
        // and should be handled in a higher layer.
        primary_commit_known_committed_ops(ctx, last_commit_num);
        next!(primary)
    } else {
        next!(do_view_change)
    }
}

fn set_latest_state(ctx: &mut VrCtx) {
    let mut largest_last_normal_view = 0;
    let mut largest_commit_num = 0;
    let mut largest_op = 0;
    let mut largest_log = Vec::new();
    // Find the latest values contained in received DoViewChange messages
    for (_, m) in ctx.quorum_messages.drain() {
        if let VrMsg::DoViewChange{op, last_normal_view, commit_num, log, ..} = m {
            if last_normal_view > largest_last_normal_view {
                largest_last_normal_view = last_normal_view;
                largest_log = log;
                largest_op = op;
            } else if last_normal_view == largest_last_normal_view && op > largest_op {
                largest_last_normal_view = last_normal_view;
                largest_log = log;
                largest_op = op;
            }
            if commit_num > largest_commit_num {
                largest_commit_num = commit_num;
            }
        }
    }
    // Compare received DoViewChange messages to the local state
    if largest_last_normal_view > ctx.last_normal_view {
        ctx.log = largest_log;
        ctx.op = largest_op;
    } else if largest_last_normal_view == ctx.last_normal_view && largest_op > ctx.op {
        ctx.log = largest_log;
        ctx.op = largest_op;
    }
    if largest_commit_num > ctx.commit_num {
        ctx.commit_num = largest_commit_num;
    }

    ctx.quorum_messages = HashMap::new();
    ctx.last_normal_view = ctx.view;
}

fn broadcast_commit_msg(ctx: &VrCtx) {
    let commit = VrMsg::Commit {
        epoch: ctx.epoch,
        view: ctx.view,
        commit_num: ctx.commit_num
    };
    broadcast(ctx, commit);
}

fn broadcast_start_view_msg(ctx: &VrCtx) {
    let start_view = VrMsg::StartView {
        epoch: ctx.epoch,
        view: ctx.view,
        op: ctx.op,
        log: ctx.log.clone(),
        commit_num: ctx.commit_num
    };
    broadcast(&ctx, start_view);
}

fn maybe_recover(ctx: &mut VrCtx, msg: VrMsg, view: u64, is_primary: bool, from: Replica) -> StateFn<VrHandler> {
    // Just store the latest seen view in ctx.view
    if view > ctx.view { ctx.view = view }
    // Is this a response from a primary?
    if is_primary && view == ctx.view {
        ctx.recovery_primary = Some((from.clone(), view))
    }
    ctx.quorum_messages.insert(from, msg);
    if super_quorum(&ctx) {
        match ctx.recovery_primary.clone() {
            Some((ref primary, ref primary_view)) if *primary_view == ctx.view => {
                let primary_response = ctx.quorum_messages.remove(primary).unwrap();
                // Always succeeds
                if let VrMsg::RecoveryResponse {epoch, op, commit_num, log, ..} = primary_response {
                    // Set our state to that of the latest primary
                    ctx.primary = Some(primary.clone());
                    ctx.epoch = epoch;
                    ctx.op = op.unwrap();
                    ctx.log = log.unwrap();
                    backup_commit_known_committed_ops(ctx, commit_num.unwrap());
                }
                // Reset recovery related variables
                ctx.quorum_messages = HashMap::new();
                ctx.recovery_nonce = None;
                ctx.recovery_primary = None;

                return next!(backup)
            },
            // We have quorum but haven't yet received a RecoveryResponse from the latest primary
            _ => return next!(recovery)
        }
    }
    next!(recovery)
}

fn maybe_send_do_view_change(ctx: &mut VrCtx) -> StateFn<VrHandler> {
    if quorum(&ctx) {
        let primary = compute_primary(ctx);
        ctx.primary = Some(primary.clone());
        if primary == ctx.me {
            return next!(do_view_change);
        }
        let do_view_change = VrMsg::DoViewChange {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            from: ctx.me.clone(),
            last_normal_view: ctx.last_normal_view,
            log: ctx.log.clone(),
            commit_num: ctx.commit_num
        };
        ctx.sender.send(Envelope::new(primary, do_view_change));
    }
    next!(view_change)
}

#[inline]
fn quorum(ctx: &VrCtx) -> bool {
    // We add 1 because we implicitly include the replica doing the check
    (ctx.quorum_messages.len() + 1) >= ctx.quorum as usize
}

// A quorum not containing one's self
#[inline]
fn super_quorum(ctx: &VrCtx) -> bool {
    ctx.quorum_messages.len() >= ctx.quorum as usize
}

// I'm sure this could be more efficient. At least the count_map could be allocated once and
// re-used.
fn get_commit_quorum(ctx: &VrCtx) -> Option<u64> {
    let mut count_map = BTreeMap::new();
    let mut highest_op = 0;
    for (_, &op) in ctx.commit_quorum.iter() {
        *count_map.entry(op).or_insert(0) += 1;
    }
    for (&op, &count) in count_map.iter() {
        if (count + 1) as u64 >= ctx.quorum {
            highest_op = op;
        }
    }
    if highest_op > 0 {
        Some(highest_op)
    } else {
        None
    }
}

fn compute_primary(ctx: &VrCtx) -> Replica {
    let index = ctx.view as usize % ctx.new_config.replicas.len();
    ctx.new_config.replicas[index].clone()
}

fn start_view_change(ctx: &mut VrCtx) -> StateFn<VrHandler> {
    let msg = VrMsg::StartViewChange {epoch: ctx.epoch,
                                      view: ctx.view,
                                      op: ctx.op,
                                      from: ctx.me.clone()};
    broadcast(ctx, msg);
    maybe_send_do_view_change(ctx)
}

fn start_recovery(ctx: &mut VrCtx) -> StateFn<VrHandler> {
    ctx.recovery_nonce = Some(Uuid::new_v4());
    let msg = VrMsg::Recovery {
        from: ctx.me.clone(),
        nonce: ctx.recovery_nonce.clone().unwrap()
    };
    broadcast(ctx, msg);
    next!(recovery)
}

// During reconfiguration if we are not up to date we need to send a get state request to all
// replicas to ensure we get the latest results.
fn broadcast_old_and_new(ctx: &VrCtx, msg: VrMsg) {
    for r in ctx.old_config.replicas.iter().cloned() {
        if ctx.me != r {
            ctx.sender.send(Envelope::new(r, msg.clone()));
        }
    }
    for r in ctx.new_config.replicas.iter().cloned() {
        if ctx.me != r {
            ctx.sender.send(Envelope::new(r, msg.clone()));
        }
    }
}

/// Wrap a VrMsg in an envelope and send to all backups
fn broadcast(ctx: &VrCtx, msg: VrMsg) {
    for backup in ctx.new_config.replicas.iter().cloned() {
        if ctx.me != backup {
            ctx.sender.send(Envelope::new(backup, msg.clone()));
        }
    }
}

#[inline]
fn idle_timeout(ctx: &VrCtx) -> bool {
    SteadyTime::now() - ctx.last_received_time > Duration::milliseconds(ctx.idle_timeout_ms)
}

#[inline]
fn primary_idle_timeout(ctx: &VrCtx) -> bool {
    SteadyTime::now() - ctx.last_received_time > Duration::milliseconds(ctx.primary_tick_ms)
}
