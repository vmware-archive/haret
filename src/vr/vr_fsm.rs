use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use uuid::Uuid;
use time::{SteadyTime, Duration};
use rand::{thread_rng, Rng};
use fsm::{FsmTypes, StateFn};
use super::replica::{Replica, VersionedReplicas};
use super::VrBackend;
use super::quorum_tracker::QuorumTracker;
use super::prepare_requests::PrepareRequests;
use super::vrmsg::VrMsg;
use super::vr_api_messages::{VrApiReq, VrApiRsp};
use super::envelope::{Envelope, Announcement, PeerEnvelope, ClientReplyEnvelope};

pub const DEFAULT_IDLE_TIMEOUT_MS: u64 = 2000;
pub const DEFAULT_PRIMARY_TICK_MS: u64 = 500;

pub type Transition = (StateFn<VrTypes>, Vec<Envelope>);

/// The internal state of the VR FSM. Note that fields are only made public for visibility,
/// debugging and testing purposes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VrCtx {
    pub me: Replica,
    pub primary: Option<Replica>,
    pub epoch: u64,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub last_received_time: SteadyTime,
    pub last_normal_view: u64,

    /// The number of replicas needed to provide quorum
    pub quorum: u64,

    /// Used to track waiting on quorums for a single message (DoViewChange, StartEpoch, ...)
    pub quorum_tracker: QuorumTracker,

    /// Used by the replica in primary state tracking quorums of Prepare requests
    pub prepare_requests: PrepareRequests,

    pub log: Vec<VrMsg>,
    pub backend: VrBackend,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas,

    /// As opposed to the Viewstamped replication revisited paper, we keep a session table instead
    /// of a client table and we do not replicate it.
    pub session_table: HashMap<Uuid, (u64, Option<VrMsg>)>,

    /// The nonce of the last sent recovery message
    pub recovery_nonce: Option<Uuid>,

    /// During recovery we must wait for a at least a quorum of replies and a primary from the latest
    /// view learned about in the quorum. We maintain track of the latest view primary heard from in
    /// `recovery_primary`
    pub recovery_primary: Option<(Replica, u64)>,
    pub recovery_primary_response: Option<VrMsg>,

    /// Backups wait `idle_timeout_ms` between messages from the primary before initiating a view
    /// change.
    pub idle_timeout_ms: u64,

    /// If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    /// message to the backups. `idle_timeout_ms` should be at least twice as large as this value.
    pub primary_tick_ms: u64,
}

impl VrCtx {
    pub fn new(me: Replica, old_config: VersionedReplicas, new_config: VersionedReplicas) -> VrCtx {
        let quorum = new_config.replicas.len() / 2 + 1;
        VrCtx {
            me: me,
            primary: None,
            epoch: new_config.epoch,
            view: 0,
            op: 0,
            commit_num: 0,
            last_received_time: SteadyTime::now(),
            last_normal_view: 0,
            quorum: quorum as u64,
            quorum_tracker: QuorumTracker::new(quorum),
            prepare_requests: PrepareRequests::new(new_config.replicas.len(),
                                                   DEFAULT_IDLE_TIMEOUT_MS),
            log: Vec::new(),
            backend: VrBackend::new(),
            old_config: old_config,
            new_config: new_config,
            session_table: HashMap::new(),
            recovery_nonce: None,
            recovery_primary: None,
            recovery_primary_response: None,
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS
        }
    }
}

#[derive(Debug, Clone)]
pub struct VrTypes;

impl FsmTypes for VrTypes {
    type Context = VrCtx;
    type Msg = VrMsg;
    type Output = Envelope;
}

macro_rules! check_epoch {
    ($ctx:ident, $msg:ident, $state:ident) => {
        if let Some(epoch) = $msg.get_epoch() {
            if epoch < $ctx.epoch {
                return next!($state);
            }
            if epoch > $ctx.epoch {
                return start_reconfiguration($ctx, $msg, epoch);
            }
        }
    }
}

macro_rules! check_view {
    ($ctx:ident, $msg:ident, $state:ident) => {
        if let Some(view) = $msg.get_view() {
            if view < $ctx.view {
                return next!($state);
            }
            if view > $ctx.view {
                return handle_later_view($ctx, $msg, view);
            }
        }
    }
}

macro_rules! handle_common {
    ($ctx:ident, $msg:ident, $state:ident) => {
        if let VrMsg::SessionClosed(session_id) = $msg {
            $ctx.session_table.remove(&session_id);
            return next!($state);
        }
        check_epoch!($ctx, $msg, $state);
        check_view!($ctx, $msg, $state);
    }
}

fn start_reconfiguration(ctx: &mut VrCtx,
                         msg: VrMsg,
                         epoch: u64) -> Transition {
    if epoch == ctx.epoch + 1 {
        match msg {
            VrMsg::Commit {commit_num, ..} => {
                return handle_commit_new_epoch(ctx, commit_num, epoch);
            },
            VrMsg::StartEpoch {op, old_config, new_config, ..} => {
                return handle_start_epoch(ctx, epoch, op, old_config, new_config);
            },
            // We have been partitioned and missed the reconfiguration. At least a quorum of new nodes
            // is normally processing requests, and the leaving replicas may have shut down.
            // We don't yet know what the 'new_config' is though. W
            _ => {
                return learn_config(ctx, epoch);
            }
        }
    }
    // We have been partitioned for one or multiple reconfigurations. We need to catch up.
    // We don't know what the old or new configs are to ask for the latest state. Therefore we wait
    // for a normal message in the new epoch with a 'from' field. We can then begin state transfer
    // using that replica.
    learn_config(ctx, epoch)
}

fn handle_later_view(ctx: &mut VrCtx, msg: VrMsg, new_view: u64) -> Transition {
    match msg.clone() {
        VrMsg::StartViewChange {ref from, ..} => {
            reset_quorum(ctx, new_view, from.clone(), msg);
            return start_view_change(ctx, Vec::new());
        },
        VrMsg::DoViewChange {ref from, ..} => {
            reset_quorum(ctx, new_view, from.clone(), msg.clone());
            return maybe_become_primary(ctx, from.clone(), msg);
        },
        VrMsg::StartView{..} => {
            return become_backup(ctx, msg);
        },
        _ => start_state_transfer_new_view(ctx, new_view)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////

/// Start the replica as an initial member of a newly created tenant
pub fn startup_new_tenant(ctx: &mut VrCtx, _msg: VrMsg) -> Transition {
    ctx.view += 1;
    start_view_change(ctx, Vec::new())
}

pub fn startup_recovery(ctx: &mut VrCtx, _msg: VrMsg) -> Transition {
    start_recovery(ctx)
}

pub fn startup_reconfiguration(ctx: &mut VrCtx, _msg: VrMsg) -> Transition {
    start_state_transfer_transitioning(ctx)
}

/// This replica is currently the primary replica operating normally
pub fn primary(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, primary);
    let msg2 = msg.clone();
    match msg {
        VrMsg::ClientRequest {op, session_id, request_num} => {
            let output =  handle_client_request(ctx, msg2, op, session_id, request_num);
            next!(primary, output)
        },
        VrMsg::PrepareOk {view, op, ref from, ..} if view == ctx.view && op > ctx.commit_num => {
            let output = maybe_commit(ctx, op, from.clone());
            next!(primary, output)
        },
        VrMsg::Tick => {
            if ctx.prepare_requests.expired() {
                return reset_and_start_view_change(ctx);
            }
            if primary_idle_timeout(ctx) {
                let output = broadcast_commit_msg(ctx);
                return next!(primary, output);
            }
            next!(primary)
        },
        VrMsg::GetState{..} => {
            let output = send_new_state(ctx, msg);
            next!(primary, output)
        },
        VrMsg::Recovery{..} => {
            let output = send_primary_recovery_response(ctx, msg);
            next!(primary, output)
        },
        VrMsg::Reconfiguration{..} => handle_reconfig(ctx, msg),
        _ => next!(primary)
    }
}

/// This replica is currently a backup operating normally
pub fn backup(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, backup);
    let msg2 = msg.clone();
    match msg {
        VrMsg::Prepare {op, ..} if op == ctx.op + 1 => {
            let output = handle_normal_prepare(ctx, msg);
            next!(backup, output)
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
            let output = send_new_state(ctx, msg2);
            next!(backup, output)
        },
        VrMsg::Recovery{..} => {
            maybe_send_recovery_response(ctx, msg)
        },
        _ => {
            println!("Re-entering backup state {:?}", ctx.me.name);
            next!(backup)
        }
    }
}

/// This is the state of the primary after it has sent a Prepare message containing the client
/// reconfiguration request.
pub fn transitioning_wait_for_prepare_ok(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, transitioning_wait_for_prepare_ok);
    match msg {
        VrMsg::PrepareOk {op, ref from, ..} if op == ctx.op => {
            maybe_commit_reconfig(ctx, op, from.clone())
        },
        VrMsg::Tick => {
            // If we haven't received quorum of PrepareOk in time then re-broadcast Prepare
            if ctx.prepare_requests.expired() {
                let output = rebroadcast_reconfig(ctx);
                return next!(transitioning_wait_for_prepare_ok, output);
            }
            next!(transitioning_wait_for_prepare_ok)
        }
        _ => next!(transitioning_wait_for_prepare_ok)
    }
}

pub fn transitioning_wait_for_new_state(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, transitioning_wait_for_new_state);
    match msg {
        VrMsg::NewState {op, ..} if op >= ctx.new_config.op => {
            let mut output = vec![set_state(ctx, msg)];
            if replicas_to_replace(ctx).contains(&ctx.me) {
                return next!(leaving, output)
            }
            output.extend_from_slice(&send_epoch_started(ctx));
            if ctx.me == compute_primary(ctx) {
                return next!(primary, output)
            }
            next!(backup, output)
        },
        VrMsg::Tick => {
            // If we haven't gotten a NewState in time then re-broadcast GetState
            if idle_timeout(ctx) {
                return start_state_transfer_transitioning(ctx)
            }
            next!(transitioning_wait_for_new_state)
        },
        _ => next!(transitioning_wait_for_new_state)
    }
}

/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
pub fn leaving(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    match msg {
        // TODO: What if epoch > ctx.epoch ?
        VrMsg::EpochStarted {epoch, ref from, ..} if epoch == ctx.epoch => {
            ctx.quorum_tracker.insert(from.clone(), msg.clone());
            if ctx.quorum_tracker.has_quorum() {
                clear_quorum_tracker(ctx);
                let output = vec![Envelope::Announcement(Announcement::Stop(ctx.me.clone()))];
                return next!(shutdown, output)
            }
            next!(leaving)
        },
        VrMsg::Tick => {
            let timeout = u64_to_duration(ctx.idle_timeout_ms);
            if ctx.quorum_tracker.is_expired(&timeout) {
                let output = broadcast_start_epoch(ctx);
                return next!(leaving, output);
            }
            next!(leaving)
        }
        _ => next!(leaving)
    }
}

/// This replica has already told the dispatcher to shut it down. It just waits in this state and
/// doesn't respond to any messages from this point out.
pub fn shutdown(_ctx: &mut VrCtx, _: VrMsg) -> Transition {
    next!(shutdown)
}

pub fn recovery(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    check_epoch!(ctx, msg, recovery);
    match msg {
        VrMsg::RecoveryResponse {view,
                                 ref op,
                                 ref nonce,
                                 ref from, ..} if Some(*nonce) == ctx.recovery_nonce =>
            maybe_recover(ctx, msg.clone(), view, op.is_some(), from.clone()),
        VrMsg::Tick => {
            let timeout = u64_to_duration(ctx.idle_timeout_ms);
            if ctx.quorum_tracker.is_expired(&timeout) {
                start_recovery(ctx)
            } else {
                next!(recovery)
            }
        },
        _ => next!(recovery)
    }
}

/// A replica is in this state during view change. The replica remains in this state until view
/// change is completed unless it is the proposed primary, which transitions to the `wait_for_do_view_change`
/// state on receipt of a `DoViewChange` message.
pub fn view_change(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, view_change);
    match msg.clone() {
        VrMsg::StartViewChange{ref from, ..} => {
            ctx.quorum_tracker.insert(from.clone(), msg.clone());
            maybe_send_do_view_change(ctx, Vec::new())
        },
        VrMsg::DoViewChange {view, ref from, ..} => {
            reset_quorum(ctx, view, from.clone(), msg.clone());
            return maybe_become_primary(ctx, from.clone(), msg);
        },
        VrMsg::StartView{..} => {
            return become_backup(ctx, msg);
        },
        VrMsg::Tick => {
            // We haven't changed views yet. The new primary must be down. Try again.
            if ctx.quorum_tracker.is_expired(&u64_to_duration(ctx.idle_timeout_ms)) {
                reset_and_start_view_change(ctx)
            } else {
                next!(view_change)
            }
        },
        VrMsg::Recovery {ref from, ..}  => {
            // We don't handle recovery messages in view_change state. However, if we receive one
            // from the primary for this view we know the election will never complete and trigger a
            // new view change immediately rather than waiting. It's possible this is an old
            // recovery message lost in the network, but an extra view change is still safe.  It's
            // impossible that the recovery message is old when using TCP as transport.
            if let Some(primary) = ctx.primary.clone() {
                if primary == *from {
                    return reset_and_start_view_change(ctx);
                }
            }
            next!(view_change)
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
pub fn wait_for_do_view_change(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, wait_for_do_view_change);
    match msg.clone() {
        VrMsg::DoViewChange{from, ..} => maybe_become_primary(ctx, from, msg),
        _ => next!(wait_for_do_view_change)
    }
}

/// When a backup realizes it's behind it enters this state
pub fn state_transfer(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, state_transfer);
    match msg {
        VrMsg::NewState {..} => {
            let output = vec![set_state(ctx, msg)];
            next!(backup, output)
        },
        _ => next!(state_transfer)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// END OF FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////

fn reset_and_start_view_change(ctx: &mut VrCtx) -> Transition {
    ctx.last_received_time = SteadyTime::now();
    ctx.view += 1;
    ctx.prepare_requests = PrepareRequests::new(ctx.new_config.replicas.len(),
                                                ctx.idle_timeout_ms);
    ctx.quorum_tracker = QuorumTracker::new(ctx.quorum as usize);
    let envelope = clear_primary(ctx);
    start_view_change(ctx, vec![envelope])
}

fn learn_config(_ctx: &mut VrCtx, _epoch: u64) -> Transition {
    // TODO: I think this is just an instance of state transfer to a new view, using the replica that informed us of a later epoch. Basically, we set our epoch than call start_state_transfer_new_view(). Talk to Justin about this. How is start_state_transfer_new_view() different from start_state_transfer_transitioning()?
    unimplemented!();
}

// TODO: This should be made into a general function that rebroadcasts the last operation. It can
// only be a reconfig or client request in the log.
fn rebroadcast_reconfig(ctx: &mut VrCtx) -> Vec<Envelope> {
    let reconfig = ctx.log[(ctx.op - 1) as usize].clone();
    if let VrMsg::Reconfiguration {session_id, client_req_num, ..} = reconfig {
        let prepare = VrMsg::Prepare {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            session_id: session_id,
            client_request_num: client_req_num,
            client_op: VrApiReq::Null,
            commit_num: ctx.commit_num
        };
        return broadcast(ctx, prepare);
    }
    unreachable!();
}

fn maybe_commit_reconfig(ctx: &mut VrCtx, op: u64, from: Replica) -> Transition {
    let output = maybe_commit(ctx, op, from);
    if !output.is_empty() {
        // We committed the reconfiguration request
        if ctx.me == compute_primary(ctx) {
            next!(primary, output)
        } else {
            next!(backup, output)
        }
    } else {
        next!(transitioning_wait_for_prepare_ok)
    }
}

fn update_ctx_for_new_epoch(ctx: &mut VrCtx, op: u64) {
    ctx.epoch += 1;
    ctx.view = 0;
    ctx.old_config = ctx.new_config.clone();
    // Get the reconfiguration from the log. This always succeeds.
    if let VrMsg::Reconfiguration {ref replicas, ..} = ctx.log[(op-1) as usize] {
        ctx.new_config = VersionedReplicas {epoch: ctx.epoch,
                                            op: ctx.op,
                                            replicas: replicas.clone()};
    }
}

fn announce_reconfiguration(ctx: &mut VrCtx) -> Envelope {
    let envelope = Envelope::Announcement(Announcement::Reconfiguration {
        tenant: ctx.me.tenant.clone(),
        old_config: ctx.old_config.clone(),
        new_config: ctx.new_config.clone()
    });
    envelope
}


/// A backup will send a recovery response if it's not to the current primary. Otherwise it will
/// start a view change.
fn maybe_send_recovery_response(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
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
        let output = vec![Envelope::Peer(PeerEnvelope::new(from, ctx.me.clone(), response))];
        return next!(backup, output);
    }
    unreachable!()
}

fn send_primary_recovery_response(ctx: &mut VrCtx, msg: VrMsg) -> Vec<Envelope> {
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
        return vec![Envelope::Peer(PeerEnvelope::new(from, ctx.me.clone(), response))];
    }
    unreachable!()
}

fn broadcast_start_epoch(ctx: &mut VrCtx) -> Vec<Envelope> {
    let msg = VrMsg::StartEpoch {
        epoch: ctx.epoch,
        op: ctx.op,
        old_config: ctx.old_config.clone(),
        new_config: ctx.new_config.clone()
    };
    broadcast(ctx, msg)
}

fn send_epoch_started(ctx: &mut VrCtx) -> Vec<Envelope>{
    let mut output = Vec::new();
    let msg = VrMsg::EpochStarted {
        epoch: ctx.epoch,
        from: ctx.me.clone()
    };

    for r in replicas_to_replace(ctx) {
        output.push(Envelope::Peer(PeerEnvelope::new(r, ctx.me.clone(), msg.clone())));
    }
    output
}

fn replicas_to_replace(ctx: &VrCtx) -> Vec<Replica> {
    let new_set = HashSet::<Replica>::from_iter(ctx.new_config.replicas.clone());
    let old_set = HashSet::<Replica>::from_iter(ctx.old_config.replicas.clone());
    old_set.difference(&new_set).cloned().collect()
}

fn send_new_state(ctx: &mut VrCtx, msg: VrMsg) -> Vec<Envelope> {
    if let VrMsg::GetState {op, from, ..} = msg {
        let mut log_tail = Vec::new();
        for i in op..ctx.op {
            log_tail.push(ctx.log[i as usize].clone());
        }
        let new_state = VrMsg::NewState {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            primary: ctx.primary.clone(),
            commit_num: ctx.commit_num,
            log_tail: log_tail
        };
        return vec![Envelope::Peer(PeerEnvelope::new(from, ctx.me.clone(), new_state))];
    }
    unreachable!()
}

fn set_state(ctx: &mut VrCtx, msg: VrMsg) -> Envelope {
    if let VrMsg::NewState {view, op, commit_num, log_tail, ..} = msg {
        ctx.view = view;
        ctx.op = op;
        clear_quorum_tracker(ctx);
        for m in log_tail {
            ctx.log.push(m);
        }
        backup_commit_known_committed_ops(ctx, commit_num);
        return set_primary(ctx);
    }
    unreachable!();
}

fn handle_start_epoch(ctx: &mut VrCtx,
                      epoch: u64,
                      op: u64,
                      old_config: VersionedReplicas,
                      new_config: VersionedReplicas) -> Transition {
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
/// leaving if they are not a member of the new group.
fn handle_commit_new_epoch(ctx: &mut VrCtx, commit_num: u64, epoch: u64) -> Transition {
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
        return start_state_transfer_transitioning(ctx)
    }
    if to_replace.contains(&ctx.me) {
        clear_quorum_tracker(ctx);
        return next!(leaving)
    }
    if ctx.me == compute_primary(ctx) {
        return next!(primary)
    } else {
        return next!(backup)
    }
}

fn handle_commit(ctx: &mut VrCtx, commit_num: u64) -> Transition {
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

fn start_state_transfer_transitioning(ctx: &mut VrCtx) -> Transition {
    ctx.last_received_time = SteadyTime::now();
    ctx.op = ctx.commit_num;
    ctx.view = 0;
    ctx.log.truncate(ctx.op as usize);
    let msg = VrMsg::GetState {
        epoch: ctx.epoch,
        view: ctx.view,
        op: ctx.op,
        from: ctx.me.clone()
    };
    let output = broadcast_old_and_new(ctx, msg);
    next!(transitioning_wait_for_new_state, output)
}

fn start_state_transfer_same_view(ctx: &VrCtx) -> Transition {
    let msg = VrMsg::GetState {
        epoch: ctx.epoch,
        view: ctx.view,
        op: ctx.op,
        from: ctx.me.clone()
    };
    let output = send_to_random_replica(ctx, msg, &ctx.new_config);
    next!(state_transfer, output)
}

fn start_state_transfer_new_view(ctx: &mut VrCtx, view: u64) -> Transition {
    ctx.op = ctx.commit_num;
    ctx.view = view;
    ctx.log.truncate(ctx.op as usize);
    start_state_transfer_same_view(ctx)
}

fn handle_normal_prepare(ctx: &mut VrCtx, msg: VrMsg) -> Vec<Envelope> {
    assert!(ctx.primary.is_some());
    if let VrMsg::Prepare { client_op, commit_num, session_id, client_request_num, ..} = msg {
        ctx.last_received_time = SteadyTime::now();
        ctx.op += 1;
        // Reconstruct Client Request, since the log stores VrMsgs
        let client_req = VrMsg::ClientRequest { op: client_op,
                                                session_id: session_id,
                                                request_num: client_request_num};
        ctx.log.push(client_req);
        let prepare_ok = VrMsg::PrepareOk { epoch: ctx.epoch,
                                            view: ctx.view,
                                            op: ctx.op,
                                            from: ctx.me.clone() };
        let output = vec![send_to_primary(ctx, prepare_ok)];
        backup_commit_known_committed_ops(ctx, commit_num);
        return output;
    }
    unreachable!()
}

fn send_to_random_replica(ctx: &VrCtx, msg: VrMsg, config: &VersionedReplicas) -> Vec<Envelope> {
    let mut rng = thread_rng();
    let mut to = ctx.me.clone();
    while to == ctx.me {
        let index = rng.gen_range(0, config.replicas.len());
        to = config.replicas[index].clone()
    }
    vec![Envelope::Peer(PeerEnvelope::new(to, ctx.me.clone(), msg))]
}

fn send_to_primary(ctx: &VrCtx, msg: VrMsg) -> Envelope {
    Envelope::Peer(PeerEnvelope::new(ctx.primary.as_ref().unwrap().clone(), ctx.me.clone(), msg))
}

/// Handle a Reconfiguration request as the primary
fn handle_reconfig(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    if let VrMsg::Reconfiguration {session_id, client_req_num, epoch, ref replicas} = msg {
        if replicas.len() < 3 {
            let errmsg = "New config must contain at least 3 replicas".to_string();
            let reply = VrMsg::ClientReply {
                epoch: ctx.epoch,
                view: ctx.view,
                request_num: client_req_num,
                value: VrApiRsp::Error {msg: errmsg}
            };
            let output = vec![send_client_reply(session_id, reply)];
            return next!(primary, output);
        }
        if epoch < ctx.epoch || epoch > ctx.epoch {
            let errmsg = "Reconfiguration attempted with incorrect epoch".to_string();
            let reply = VrMsg::ClientReply {
                epoch: ctx.epoch,
                view: ctx.view,
                request_num: client_req_num,
                value: VrApiRsp::Error {msg: errmsg}
            };
            let output = vec![send_client_reply(session_id, reply)];
            return next!(primary, output);
        }
        let output = handle_client_request(ctx, msg.clone(), VrApiReq::Null, session_id, client_req_num);
        return next!(transitioning_wait_for_prepare_ok, output)
    }
    unreachable!()
}

/// Handle a client request as the primary
fn handle_client_request(ctx: &mut VrCtx,
                         msg: VrMsg,
                         client_op: VrApiReq,
                         session_id: Uuid,
                         req_num: u64) -> Vec<Envelope>
{
    if let Some(envelopes) = sent_last_response(ctx, &session_id, req_num) {
        return envelopes;
    }
    ctx.last_received_time = SteadyTime::now();
    ctx.session_table.insert(session_id, (req_num, None));
    ctx.op += 1;
    ctx.log.push(msg);
    let prepare = VrMsg::Prepare { epoch: ctx.epoch,
                                   view: ctx.view,
                                   op: ctx.op,
                                   session_id: session_id,
                                   client_request_num: req_num,
                                   client_op: client_op,
                                   commit_num: ctx.commit_num };
    ctx.prepare_requests.new_prepare(ctx.op);
    broadcast(ctx, prepare)
}

fn sent_last_response(ctx: &mut VrCtx, session_id: &Uuid, req_num: u64) -> Option<Vec<Envelope>> {
    if let Some(&(last_req_num, ref maybe_last_response)) = ctx.session_table.get(&session_id) {
        if last_req_num == req_num && maybe_last_response.is_some() {
            // Client replies must be sent with the latest epoch and view. This may be different
            // from when the operation was actually committed and stored in the session_table.
            let last_response = maybe_last_response.clone().unwrap();
            if let VrMsg::ClientReply {request_num, value, ..} = last_response {
                let reply = VrMsg::ClientReply {epoch: ctx.epoch,
                                                view: ctx.view,
                                                request_num: request_num,
                                                value: value};
                return Some(vec![send_client_reply(session_id.clone(), reply)]);
            }
        } else if last_req_num >= req_num {
            return Some(Vec::new());
        }
    }
    None
}

fn send_client_reply(session_id: Uuid, reply: VrMsg) -> Envelope {
    Envelope::ClientReply(ClientReplyEnvelope {to: session_id, msg: reply})
}

fn become_backup(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    if let VrMsg::StartView {view, op, log, commit_num, ..} = msg {
        ctx.view = view;
        ctx.op = op;
        ctx.log = log;
        ctx.prepare_requests = PrepareRequests::new(ctx.new_config.replicas.len(),
                                                    ctx.idle_timeout_ms);
        ctx.quorum_tracker = QuorumTracker::new(ctx.quorum as usize);
        let mut output = Vec::new();
        output.push(set_primary(ctx));
        output.extend_from_slice(&send_prepare_ok_for_uncommitted_ops(ctx));
        backup_commit_known_committed_ops(ctx, commit_num);
        ctx.last_normal_view = ctx.view;
        return next!(backup, output);
    }
    unreachable!();
}

fn backup_commit_known_committed_ops(ctx: &mut VrCtx, new_commit_num: u64) {
    for i in ctx.commit_num..new_commit_num {
        let msg = ctx.log[i as usize].clone();
        match msg {
            VrMsg::ClientRequest {op, ..} => {
                ctx.backend.call(i+1, op);
            },
            VrMsg::Reconfiguration {replicas, ..} => {
                ctx.old_config = ctx.new_config.clone();
                ctx.new_config = VersionedReplicas {epoch: ctx.epoch, op: i+1, replicas: replicas};
                ctx.backend.call(i+1, VrApiReq::Null);
            },
            _ => ()
        }
    }
    ctx.commit_num = new_commit_num;
}

fn primary_commit_known_committed_ops(ctx: &mut VrCtx, last_commit_num: u64) -> Vec<Envelope> {
    let mut output = Vec::new();
    for i in last_commit_num..ctx.commit_num {
        let msg = ctx.log[i as usize].clone();
        if let VrMsg::ClientRequest {op, session_id, request_num} = msg.clone() {
            let rsp = ctx.backend.call(i+1, op.clone());
            let reply = VrMsg::ClientReply {epoch: ctx.epoch,
                                            view: ctx.view,
                                            request_num: request_num,
                                            value: rsp};
            if store_client_reply(ctx, &session_id, request_num, &reply) {
                let envelope = send_client_reply(session_id, reply);
                output.push(envelope);
            }
        }
        if let VrMsg::Reconfiguration {session_id, client_req_num, epoch, ..} = msg {
            let rsp = ctx.backend.call(i+1, VrApiReq::Null);
            let reply = VrMsg::ClientReply {
                epoch: epoch,
                view: 0,
                request_num: client_req_num,
                value: rsp
            };
            if store_client_reply(ctx, &session_id, client_req_num, &reply) {
                let envelope = send_client_reply(session_id, reply);
                output.push(envelope)
            }
            update_ctx_for_new_epoch(ctx, i+1);
            output.push(announce_reconfiguration(ctx));
            output.extend_from_slice(&broadcast_commit_msg_old(ctx));
            output.extend_from_slice(&send_epoch_started(ctx));
        }
    }
    output
}

/// Store a client reply in the session table if the session is current for the primary
/// Returns true if the client reply was stored in the session table, false otherwise
fn store_client_reply(ctx: &mut VrCtx, session_id: &Uuid, req_num: u64, reply: &VrMsg) -> bool {
    if let Some(&mut(ref mut req, ref mut rpy) ) = ctx.session_table.get_mut(&session_id) {
        *req = req_num;
        *rpy = Some(reply.clone());
        true
    } else {
        false
    }
}

fn send_prepare_ok_for_uncommitted_ops(ctx: &VrCtx) -> Vec<Envelope> {
    let mut output = Vec::new();
    for i in ctx.commit_num..ctx.op {
        let msg = VrMsg::PrepareOk {epoch: ctx.epoch, view: ctx.view, op: i+1, from: ctx.me.clone()};
        output.push(send_to_primary(ctx, msg));
    }
    output
}

fn clear_quorum_tracker(ctx: &mut VrCtx) {
    ctx.quorum_tracker = QuorumTracker::new(ctx.quorum as usize);
}

fn reset_quorum(ctx: &mut VrCtx, view: u64, from: Replica, msg: VrMsg) {
    ctx.last_received_time = SteadyTime::now();
    ctx.view = view;
    ctx.prepare_requests = PrepareRequests::new(ctx.new_config.replicas.len(),
                                                ctx.idle_timeout_ms);
    ctx.quorum_tracker = QuorumTracker::new(ctx.quorum as usize);
    ctx.quorum_tracker.insert(from, msg);
}

/// Returns true if the op was committed
fn maybe_commit(ctx: &mut VrCtx, op: u64, from: Replica) -> Vec<Envelope> {
    ctx.prepare_requests.insert(op, from.clone());
    if ctx.prepare_requests.has_quorum(op) {
        let last_commit_num = ctx.commit_num;
        ctx.commit_num = op;
        ctx.prepare_requests.remove(op);
        primary_commit_known_committed_ops(ctx, last_commit_num)
    } else {
        Vec::new()
    }

}

fn maybe_become_primary(ctx: &mut VrCtx, from: Replica, msg: VrMsg) -> Transition {
    ctx.quorum_tracker.insert(from, msg);
    if ctx.quorum_tracker.has_quorum() {
        let mut output = Vec::new();
        let last_commit_num = ctx.commit_num;
        set_latest_state(ctx);
        output.extend_from_slice(&broadcast_start_view_msg(&ctx));
        output.extend_from_slice(&primary_commit_known_committed_ops(ctx, last_commit_num));
        clear_quorum_tracker(ctx);
        output.push(set_primary(ctx));
        println!("Elected {:?} as primary of view {}", ctx.primary, ctx.view);
        // We may have just committed a Reconfiguration request. Check to see if we are still the
        // primary
        if ctx.me == compute_primary(ctx) {
            next!(primary, output)
        } else {
            next!(backup, output)
        }
    } else {
        next!(wait_for_do_view_change)
    }
}

fn clear_primary(ctx: &mut VrCtx) -> Envelope{
    ctx.primary = None;
    Envelope::Announcement(Announcement::ClearPrimary(ctx.me.tenant))
}

fn set_primary(ctx: &mut VrCtx) -> Envelope {
    let primary = compute_primary(ctx);
    set_primary_(ctx, primary)
}

fn set_primary_(ctx: &mut VrCtx, primary: Replica) -> Envelope {
    ctx.primary = Some(primary.clone());
    Envelope::Announcement(Announcement::NewPrimary(primary))
}

fn set_latest_state(ctx: &mut VrCtx) {
    let mut largest_last_normal_view = 0;
    let mut largest_commit_num = 0;
    let mut largest_op = 0;
    let mut largest_log = Vec::new();
    // Find the latest values contained in received DoViewChange messages
    for (_, m) in ctx.quorum_tracker.drain() {
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

    ctx.last_normal_view = ctx.view;
    clear_quorum_tracker(ctx);
}

fn broadcast_commit_msg(ctx: &VrCtx) -> Vec<Envelope> {
    let commit = VrMsg::Commit {
        epoch: ctx.epoch,
        view: ctx.view,
        commit_num: ctx.commit_num
    };
    broadcast(ctx, commit)
}

fn broadcast_commit_msg_old(ctx: &VrCtx) -> Vec<Envelope> {
    let commit = VrMsg::Commit {
        epoch: ctx.epoch,
        view: ctx.view,
        commit_num: ctx.commit_num
    };
    broadcast_old(ctx, commit)
}

fn broadcast_start_view_msg(ctx: &VrCtx) -> Vec<Envelope> {
    let start_view = VrMsg::StartView {
        epoch: ctx.epoch,
        view: ctx.view,
        op: ctx.op,
        log: ctx.log.clone(),
        commit_num: ctx.commit_num
    };
    broadcast(&ctx, start_view)
}

fn maybe_recover(ctx: &mut VrCtx, msg: VrMsg, view: u64, is_primary: bool, from: Replica) -> Transition {
    // Just store the latest seen view in ctx.view
    if view > ctx.view { ctx.view = view }
    // Is this a response from a primary?
    if is_primary && view == ctx.view {
        ctx.recovery_primary = Some((from.clone(), view));
        ctx.recovery_primary_response = Some(msg.clone());
    }
    let mut output = Vec::new();
    ctx.quorum_tracker.insert(from, msg);
    if ctx.quorum_tracker.has_super_quorum() {
        match ctx.recovery_primary.clone() {
            Some((_, ref primary_view)) if *primary_view == ctx.view => {
                // Always succeeds
                if let VrMsg::RecoveryResponse {epoch, op, commit_num, log, ..} = ctx.recovery_primary_response.take().unwrap() {
                    // Set our state to that of the latest primary
                    output.push(set_primary(ctx));
                    ctx.epoch = epoch;
                    ctx.op = op.unwrap();
                    ctx.log = log.unwrap();
                    backup_commit_known_committed_ops(ctx, commit_num.unwrap());
                }
                clear_quorum_tracker(ctx);
                // Reset recovery related variables
                ctx.recovery_nonce = None;
                ctx.recovery_primary = None;
                ctx.recovery_primary_response = None;

                return next!(backup, output)
            },
            // We have quorum but haven't yet received a RecoveryResponse from the latest primary
            _ => return next!(recovery)
        }
    }
    next!(recovery)
}

fn maybe_send_do_view_change(ctx: &mut VrCtx, mut output: Vec<Envelope>) -> Transition {
    if ctx.quorum_tracker.has_quorum() {
        output.push(set_primary(ctx));
        let primary = ctx.primary.clone().unwrap();
        if primary == ctx.me {
            return next!(wait_for_do_view_change, output);
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
        output.push(Envelope::Peer(PeerEnvelope::new(primary, ctx.me.clone(), do_view_change)));
    }
    next!(view_change, output)
}

fn compute_primary(ctx: &VrCtx) -> Replica {
    let index = ctx.view as usize % ctx.new_config.replicas.len();
    ctx.new_config.replicas[index].clone()
}

fn start_view_change(ctx: &mut VrCtx, mut output: Vec<Envelope>) -> Transition {
    let msg = VrMsg::StartViewChange {epoch: ctx.epoch,
                                      view: ctx.view,
                                      op: ctx.op,
                                      from: ctx.me.clone()};
    output.extend(broadcast(ctx, msg));
    maybe_send_do_view_change(ctx, output)
}

fn start_recovery(ctx: &mut VrCtx) -> Transition {
    clear_quorum_tracker(ctx);
    ctx.recovery_nonce = Some(Uuid::new_v4());
    ctx.recovery_primary = None;
    ctx.recovery_primary_response = None;
    let msg = VrMsg::Recovery {
        from: ctx.me.clone(),
        nonce: ctx.recovery_nonce.clone().unwrap()
    };
    let output = broadcast(ctx, msg);
    next!(recovery, output)
}

// During reconfiguration if we are not up to date we need to send a get state request to all
// replicas to ensure we get the latest results.
fn broadcast_old_and_new(ctx: &VrCtx, msg: VrMsg) -> Vec<Envelope> {
    let mut output = Vec::new();
    for r in ctx.old_config.replicas.iter().cloned() {
        if ctx.me != r {
            output.push(Envelope::Peer(PeerEnvelope::new(r, ctx.me.clone(), msg.clone())));
        }
    }
    for r in ctx.new_config.replicas.iter().cloned() {
        if ctx.me != r {
            output.push(Envelope::Peer(PeerEnvelope::new(r, ctx.me.clone(), msg.clone())));
        }
    }
    output
}

fn broadcast_old(ctx: &VrCtx, msg: VrMsg) -> Vec<Envelope> {
    let mut output = Vec::new();
    for replica in ctx.new_config.replicas.iter().cloned() {
        if ctx.me != replica {
            output.push(Envelope::Peer(PeerEnvelope::new(replica, ctx.me.clone(), msg.clone())));
        }
    }
    output
}

/// Wrap a VrMsg in an envelope and send to all replicas
fn broadcast(ctx: &VrCtx, msg: VrMsg) -> Vec<Envelope> {
    let mut output = Vec::new();
    for replica in ctx.new_config.replicas.iter().cloned() {
        if ctx.me != replica {
            output.push(Envelope::Peer(PeerEnvelope::new(replica, ctx.me.clone(), msg.clone())));
        }
    }
    output
}

#[inline]
/// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
/// here since the values are so small.
fn idle_timeout(ctx: &VrCtx) -> bool {
    SteadyTime::now() - ctx.last_received_time > Duration::milliseconds(ctx.idle_timeout_ms as i64)
}

#[inline]
/// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
/// here since the values are so small.
fn primary_idle_timeout(ctx: &VrCtx) -> bool {
    SteadyTime::now() - ctx.last_received_time > Duration::milliseconds(ctx.primary_tick_ms as i64)
}

pub fn u64_to_duration(timeout: u64) -> Duration {
    Duration::milliseconds(timeout as i64)
}

