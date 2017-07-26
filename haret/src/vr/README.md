This README describes the organization of the [Viewstamped Replication
Revisited](http://pmg.csail.mit.edu/papers/vr-revisited.pdf) protocol code in this
directory.

# Files

 * `replica.rs` - This file contains a representation of a VR replica that contains a `state` member
   variable that is the current state of the VR protocol. `Replica` implements a rabble process that
   accepts a Haret [Msg](https://github.com/vmware/haret/blob/master/haret/src/msg.rs).  It extracts
   out the VR related messages in its `handle` function and then transitions to the next state based
   on the message. It also handles administrative `GetReplicaState` messages that return the
   internal state to the admin user for help debugging.
 * `vr_ctx.rs` - Contains `VrCtx` which is the internal data state of a replica that exists in all states
   of the state machine. It is a component in each of the state Enums making up the VR protocol.
 * `vr_fsm.rs` - This file enumerates all states in the protocol implementation via the `VrState`
   enum as well as related traits and macros. Each state is defined in its own file in the
   `states` subdirectory. Note that these states are more fine grained than those in the paper. This
   is because each state handles messages differently in practice. For example the `normal` state in
   the paper is broken down into `Primary` and `Backup` states in the implementation, and the `view
   change` state is broken down into `StartViewChange`, `DoViewChange`, and `StartView` states.
 * `vr_msg.rs` - This file contains the `VrMsg` enum which enumerates all messages handled by the
   VRR protocol states. The messages themselves are defined with the `msg!` macro and are paramters
   to each variant in the enum. This allows them to be passed individually to different state
   related handler functions without a bunch of extra matching on the `VrMsg` and resulting required
   `unreachable!` macros when a state is known a priori from an earlier match in a calling function.

# States
Each state in the protocol lives in the `states` subdirectory and is created via the `state!` macro
defined in `vr_fsm.rs`. States must implement the `Transition` trait, also defined in `vr_fsm.rs`.
The `handle` method is called for a given state when that state is the current state of the replica.
Each state handles messages differently, and an attempt was made for all state specific code to live
in the file of each state.

Note that the `handle` method take `self` by value. It returns a new state wrapped in a `VrState`
via an implementation of the [From](https://doc.rust-lang.org/std/convert/trait.From.html) trait
for VrState for each concrete state. This `From` implementation is created for all states in the
`state!` macro. Due to this `From` usage, a common pattern when transitioning back to the same
state is to return `self.into()`. Transitioning to other states is often done via state specific
wraper functions, such as `StartViewChange::enter(..)`.

The handle function also contains the message causing the state transition, the sender, a
correlation id and a mutable Vec where any outgoing messages can be placed. This style is used so
that there are no side effects in the functions. All output messages are placed on the Vec and
processed by the [Rabble
executor](https://github.com/andrewjstone/rabble/blob/master/src/executor/executor.rs#L64-L69) or
a [test scheduler](https://github.com/vmware/haret/blob/master/haret/tests/utils/scheduler.rs).

Functions shared among implementation states live in the `states/common` subdirectory in files named
after the VR protocol states in the paper. `Primary` and `Backup` states share functionality in
`normal.rs`, while `StartViewChange`, `DoViewChange`, and `StartView` share functionality in
`view_change.rs`.

Lastly, in some instances states have the same type names as messages. In places where this is
ambiguous, the messages are prefixed with `vr_msg`, such as `vr_msg::StartViewChange`.
