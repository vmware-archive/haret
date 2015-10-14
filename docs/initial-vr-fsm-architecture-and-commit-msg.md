Add FSM implementation of VR Protocol

## Overview and Caveats
This is a very large commit consisting of VR Protocol code, management code for the VR protocol
FSMs, and deterministic testing tools for testing VR replica groups. The VR protocol code is not
100% complete and tested yet. Specifically, the code for normal operation, view change, and recovery
of individual crashed nodes is well tested and robust. Testing of operation that includes dropped
messages and reconfiguration is not complete. In fact, reconfiguration itself is not yet complete
as it does not deal with multiple reconfigurations during long partitions as described
[here](https://github.com/vmware/v2r2/blob/master/docs/vr-made-live-notes.md#long-lasting-partitions-and-reconfiguration).
Specifically, the `learn_config()` function is unimplemented
[here](https://github.com/vmware/v2r2/compare/feature/vr-fsm?expand=1#diff-39af4873f4480336a772a9e882edbcd5R489).

The VR protocol itself is implemented in FSMs using a currently private [FSM
library](https://github.com/andrewjstone/fsm/tree/feature/fsm-no-dynamic-channels). Since cargo
cannot handle private libraries, just pulling v2r2 and running make no longer works. Users must pull
the `fsm` library and then update `Cargo.toml` to point to the location on their filesystem, as shown
[here](https://github.com/vmware/v2r2/compare/feature/vr-fsm?expand=1#diff-80398c5faae3c069e4e6aa2ed11b28c0R16).
Additionally the user must ensure that they are on the `feature/fsm-no-dynamic-channels` branch of
`fsm`.

The FSMs used in implementing the VR protocol make the transport for messages transparent. This
allows complete determinism by running a replica set inside a single thread. We can thus force
linearizability and repeatability of testing, which will be discussed more below. The essential
caveat of this is that we have not yet hooked up TCP and routing of messages to different nodes.
This results in little testing of timeout lengths, and insurance that we always properly set the
message receipt time so that we can trigger both `Commit` messages and view changes. In the tests we
use a timeout of zero everywhere and trigger ticks manually to always force a commit or view change,
as will be described as well in more detail below.

As described above, the VR protocol is not hooked up to the event loop and is not usable in a
production manner yet. In the pull request there is code that changes the `vr_api` messages as well
as `v2r2-cli-client.rs`. While these are external messages and interfaces, these specific changes
were only made to satisfy the type checker and can be mostly ignored for now.

#### A note about performance

This implementation is a naive implementation that uses log shipping. It is intended to be
functional, but can obviously be made much more efficient with methods described in the VR Revisited
paper. Furthermore, there is a lot of excess copying of messages being performed, even though said
messages are immutable. Future performance enhancements should use reference counted pointers when
messages are shared within a thread, and atomic reference counted pointers when shared across
threads.

## FSMs
Finite state machines are used to implement the VR protocol inside v2r2 and are the core of the
system. While the FSM library contains an event loop implementation with 4 byte framing of tcp
messages, as well as a dynamically typed channel trait, none of that is relevant in this PR. This
discussion will concentrate solely on the FSM implementation.

As Rust is a statically typed language, it provides a non-blocking static `channel` in the standard
library to communicate between threads. Since FSMs implement a *finite* state machine, this implies
that the number of different messages handled by an FSM is fixed. A fixed number of messages does
not require a datatype that is **open for extension**, and therefore an
[enum](https://doc.rust-lang.org/nightly/book/enums.html) is the perfect structure to use to
represent the messages handled by an fsm. This allows using a standard static channel and is the
reason for the `feature/fsm-no-dynamic-channels` branch.

While a closed type is ideal for messages being received and sent by FSMs, this is not necessarily
true of messages sent external to FSMs. It's nice to have those be open for extension so that we can
add new control messages without changing code all over the system. It also allows for a standard
rate limited channel (using dynamic types) so that we don't overload the system. It is for this case
that we provide a HeuristicChannel type in the FSM library. The intention is that a boundary layer
will rate limit network requests of all types by sending them to an fsm control layer over a
HeuristicChannel which can drop messages when in overload. Those messages will then be converted to
the proper type that can be handled by an FSM or dropped if not applicable to the given FSM. This
strategy allows for isolation of the protocol code inside FSMs from the rest of the system and
greatly simplifies testing.

The FSMs themselves implement the [Fsm
trait](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/fsm.rs#L16).
Implementations are parameterized by an implementation of the `FsmHandler` trait. Currently there are
2 types of FSMs: [Local
Fsms](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/local_fsm.rs)
that run in the context of the creator, and [Threaded
Fsms](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/threaded_fsm.rs)
which spawn a thread on construction and communicate over two typed channels that are hidden behind
the FSM trait implementation. Additionally, since the `new()` method is not part of the `FsmHandler`
trait, other senders and receivers can be passed in to communicate with external parties. This is
used, with restraint, to implement the VR FSM Handlers as described below.

FSMs are objects that drive the transition of the FSM Handler States and update the application
state of a given FSM Handler. FSM Handlers implement the [FsmHandler
trait](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/fsm.rs#L31), to construct application and
protocol specific FSMs. The associated type, `Context`, is the internal state of the FSM. The
associated type, `Msg`, is the message received by the FSM that drives the transition between states
and causes update to the context. Each FSM starts off in an initial state which is returned from the
implmentation of the FsmHandler `initial_state()` function. This function returns a `StateFn<Self>`
structure that defines a `state` in a finite state machine. The structure of this is
somewhat obtuse, so let's defined exactly what it's for and why it's structured this way.

Ideally, we'd like to define a state as a function pointer which points to a 2-arity function with
the signature `Fn(&mut T::Context, T::Msg) -> Fn(&mut T::Context, T::Msg)`. This means when we set the
initial state we end up with a pointer to some function that takes the initial state (`T::Context`)
and a message (`T::Msg`) meant to transition the state machine. When the function is called it will
mutate the context and return the next state as another function pointer with the same
signature. However, if you look closely you'll notice that the above signature is incomplete. It
takes a context and message that returns a 2-arity function, but that 2-arity function doesn't
return anything! It needs to also return a 2-arity function and so on, forever. In order to
recursively define function signatures like this, Rust requires that we [wrap them in a
struct](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/fsm.rs#L26).
While a bit tedious, it allows simplicity in FsmHandler implementation signatures and also allows us
to embed other data inside the returned struct. In this particular case we also store a static
string which is the name of the state. This allows us to define an initial state function in the
following manner: `pub fn startup(ctx: &mut MyCtx, msg: MyMsg) -> StateFn<MyHandler>`.

Ok, so now we know how to define our states in a given state machine. They are just `StateFn<T>`
structs. But we really don't want to have to manually construct this structure each time we want to
return the next state. That is a bit tedious and requires duplication with something like the following:
`StateFn("somestate", somestate)`. Instead we use the [next!
macro](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/fsm.rs#L4) like
this: `next!(somestate)`. That will construct and return the proper StateFn. Note, importantly, that
when implementing an `FsmHandler` we don't need to define the states ahead of time. They are simply
functions of the proper signature. This prevents extra boilerplate requiring the definition of the
states twice. However, since other functions in the same module can have the same signature, it
becomes unclear which functions are states in the state machine. For now, in v2r2, clustering of the
state functions and opening and closing comments suffice. However, it may be better to later on have
the state functions be in their own module or somehow declare them for clarity. That's an open
question. Eventually, it should be fairly easy to write a tool to generate the state diagrams.

The `fsm` library contains a file
[fsm_check.rs](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/src/fsm_check.rs) that is used to check constraints against a running FSM in a test scenario. There are 4 types of constraints: preconditions, postconditions, invariants and transitions. Not much more will be said about these constraints, other than they are tested and can be used in the [following manner](https://github.com/andrewjstone/fsm/blob/feature/fsm-no-dynamic-channels/tests/utils/bowl_fsm.rs#L139-L160). Fsm constraint checking is local to a given FSM and cannot verify global invariants across a cluster of FSMs. It is therefore not quite powerful enough to test the VR protocol, although it is still useful. This PR does not use the FSM constraint checking in it's tests, but uses another type of deterministic model checking that can verify global invariants as discussed in the section on testing. It's expected that we will add FSM constraints later and use them when constructing state transition diagrams, as well as to add an extra layer of protection from logic mistakes.

## The VR FSM

The VR Protocol is implemented purely inside a VR FSM Handler. Each instantiated `Fsm` parameterized by a
[VrHandler](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/vr_fsm.rs#L141-L150) acts as a
single replica in a replica group. Each replica group is a `tenant` (more on tenants later). A
[VrCtx](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/vr_fsm.rs#L27-L77) maintains the
internal state of an instantiated replica. A
[VrMsg](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/messages.rs#L6-L106) represents the messages received by the FSM
that drive state transitions. All messages in the enum except for `Tick`, `ClientRequest{..}`, and
`ClientReply{..}` are sent between replicas to implement the VR protocol with the help of a
`Dispatcher`, which will be described in it's own section.

VR FSMs start out in the `startup` state. They are managed and started by the dispatcher, and are
started in one of three states: `InitialConfig`, `Recovery`, or `Reconfiguration`, as described in
this [comment](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/vr_fsm.rs#L249-L251). On an
InitialConfig, a view change will be started when the replica receives a tick (immediately sent by
the dispatcher). As long as all replicas are in communication, a single replica will be elected
primary for view 1. This primary is chosen determnisitically following this [simple
algorithm](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/vr_fsm.rs#L1108-L1111).

The rest of the code in the ``vr_fsm`` module follows the VR Revisited paper as closely as possible and
it should be fairly straightforward if one follows the messages sent and the state transitions using
`next!(..)` calls. A few contraditctions should be mentioned regarding the prior statement. It may
not be super clear, but in order to decide when to handle messages with a given epoch or view, we
isolate this code into
[check_epoch!](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/vr_fsm.rs#L152-L205) and
[check_view!](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/vr_fsm.rs#L211-L243) macros
and call them in the beginning of many states. The reason macros are used is that they allow for
early returns without having to nest code unnecessarily in conditionals. When these checks are
passed, it means that both the epoch and view are current for the given message. It also abstracts
out handling for common messages. This allows each state function to only handle the messages it
directly needs to handle while not having to worry about constantly checking epochs and view
numbers.

All the states in the FSM are not covered here, but it must be noted that there are more states than
just the 4 statuses described in the paper. The idea is that this helps isolate message handling
and makes the code clearer. The specific state, and it's context, can be retrieved by calling the
`get_state()` method of the fsm. There is no member variable in context that tracks the `status` from
the paper, although it may be useful to add it. However, it is easy to deduce the status since
each state in the fsm corresponds to exactly 1 status in the paper. For example: `primary` and
`backup` states indicate `normal` status.

One last thing to note is that a VR FSM communicates on 3 different static channels in order to both
aid testing and simplify code. Each `VrCtx` has a sender for each channel that allows it to send
different messages to the outside world. These channels are not related to any channels used by the
threaded FSM which are only used to get messages in and out of the thread. The three sender
variables are as follows:
 * `sender` - This allows sending messages destined for another replica. They are sent to the
        dispatcher wrapped in an
        [Envelope](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/messages.rs#L109-L111)
        that contains the replica id. The dispatcher will route them appropriately.
 * ``client_reply_sender`` - This allows sending replies to a given client, by wrapping the reply in a
       [ClientReplyEnvelope](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/messages.rs#L124-L127)
 * ``dispatch_sender`` - This allows sending messages directly to the dispatcher. The primary case
       is to update the dispatchers knowledge of tenant configurations after a reconfiguration has
       committed and allow starting of new replicas local to the node. The dispatcher also gossip
       these reconfigurations around to reach any possibly partitioned nodes.

## The Dispatcher

Replicas in v2r2 are simply instantiated FSMs. Right now they are all local fsms and are started in
the caller's context. This means that for each node there needs to be an entity that starts and stops
the fsms, controls their execution, and manages the sending and receiving of messages between fsms
and clients on both the same node (in testing) and different nodes. This is the role of the
[Dispatcher](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/dispatcher.rs#L20).

There is one dispatcher per node in a VR cluster. On the request of a client from the admin api, a
tenant is created by calling
[create_tenant](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/dispatcher.rs#L98-L102).
This takes a set of [Raw
Replicas](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/replica.rs#L7-L10), consisting
of a replica name and a node in the cluster where that replica will live. The dispatcher then
generates tenant id (a V4 UUID) and stores the information in the [tenants
member](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/dispatcher.rs#L22) which maps the
UUID to a set of [Versioned
Replicas](https://github.com/vmware/v2r2/blob/feature/vr-fsm/src/vr/replica.rs#L46-L54). The
versioning is maintained so that when a reconfiguration occurs, the new configuration can be
gossiped around to other nodes and those nodes can then start any new replicas in the tenant, while
shutting down old ones. Note that this is safe since all reconfigurations are committed via the VR
protocol, and the versions are monotonic. Once a tenant is created, the replicas local to the node
are started, and the tenant information is gossiped around to other nodes. Note that this gossip
layer is not yet implemented. Also note that tenant ids are created by the dispatcher on a given
node where the admin request lands. The reason for this is to prevent conflict if the same replicas
are added to a tenant on a different node. The tenant id will be returned to the admin user. We will
likely add another admin command to alias a friendly name to the generated UUID and help prevent
confusion from concurrent requests, or failed and retried ones. Accidentally created tenants can
simply be deleted.

The dispatcher also manages all timer ticks sent to local replicas. These ticks are used by
primaries to determine when to send commit messages and by backups to decide when to start a view
change. Currently there is no tick generator, as all ticks are generated by test code.
The tick generator will have to run in another thread managed by the dispatcher. It's likely that
this will be the event loop provided by the FSM, since there is already support for timer ticks
there.

## Testing

The main strategy used to test the VR protocol is one of fully deterministic fuzz testing.
Messages are generated randomly and sent to a chosen replica per message. After each step up to three
types of constraints are checked. Two of them are always checked, and the third is only checked
on client requests. [VR protocol
invariants](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/vr_invariants.rs) are
invariants that should always be maintained by a replica group, such as "there is only one primary
per epoch and view". On each operation a
[model](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/model.rs) is updated and the
state of the model is compared to the actual state of the replicas. This comparison is done after
each operation is run. Finally, [Operation
invariants](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/op_invariants.rs) assert
that the response received by a client after a client request is consistent with the state of the
replicas and is in fact a correct response. This constraint is only checked after client requests.

Fuzz testing is performed by the
[Fuzzer](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L76-L85), which is
parameterized by an implementation of the [Test
trait](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L61-L74). The fuzzer
maintains a history of operations and stops when [`test.run()` returns an
`Err(_)`](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L110). Because the
history of operations to induce failure may be quite long, an attempt is made to
[shrink](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L130) the results
and find a smaller failure on a subset of those messages. The shrinking strategy will be described
below.

So far there is a [single fuzz
test](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/static_membership_fuzz.rs#L185-L191) which
tests normal operation, view change, and recovery of nodes. Since the fuzzer is generic, it runs the
callback functions provided by the test in order to [generate each
request](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/static_membership_fuzz.rs#L114),
[update the model
state](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/static_membership_fuzz.rs#L129)
before running the operation, and [run the
operation](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/static_membership_fuzz.rs#L129).
Other callbacks are used to return textual context data for later debugging. Output is saved for
each run to the `tests/output` directory. A symlink to `tests/output/<Name of Test>/current` is
created after each test run to point to the latest test data.

Specific generators for the given test are contained in the test module, [like
this](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/static_membership_fuzz.rs#L185-L191).
Reusable generators across tests are provided in the [generators.rs
module](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/generators.rs). [Test
messages](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/test_msg.rs) are the
specific requests generated by a fuzz test. They are slightly different from the actual VR messages
in that they provide more context than those messages to enable better testing. For instance,
`ViewChange(Replica)` instructs the test to send a `VrMsg::Tick` to a live backup. The `CausalMsg`
trait is used during shrinking to ensure that if we remove a message such as `Restart` we also
remove the corresponding `Crash`, since otherwise the shrunk history no longer reflects the original
history.

The final utility module provided is
[`test_setup.rs`](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/test_setup.rs),
which is used to unify test setup across different fuzz tests. The [`init_tenant()`
function](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/test_setup.rs#L7-L23)
instantiates a dispatcher and creates an initial tenant with an empty (all zeroes) uuid and 3
replicas all living on the same local node.
[`elect_initial_leader()`](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/test_setup.rs#L25-L56)
triggers an initial view change that creates `view 1` and establishes `r2` as the primary, with the
other two nodes as backups. This standardized starting state is required by the model to ensure
consistency during model checking.

Additionally, there is a unit test written before the fuzz tester that tests basic operations of the
protocol, and is much less useful. It lives
[here](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/local_vr_group.rs), and actually
does test some basics of reconfiguration, even though that is not yet fully vetted.

#### Shrinking
Modeled after QuickCheck in Haskell and Erlang, Shrinking allows a reduction of the original failing
test case to a smaller example that also exhibits a failure, in order to ease debugging by a human.
In the case of our fuzz testing, we shrink the history of generated messages and try to find a
smaller history consisting of a subset of the original messages that generates a failure.

Because the standard `assert!` and `assert_eq!` macros panic the thread, we can not use them during
fuzz testing. We need to be indicate failure without panicing so that we can shrink the history. To
enable this the
[`safe_assert!`](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L14-L30),
[`safe_assert_eq!`](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L14-L30)
and [fail!](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L14-L30) macros
are provided. These macros record the exact location of the failure and will report it to the user.

The shrinking strategy used is naive, but turns out to be fast enough in practice. Rarely does
shrinking take more than a second or 2 for a history of 1000 messages. Shrinking begins by
[iteratively removing smaller and smaller chunks of the
history](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L14-L30) and
[re-running the
test](https://github.com/vmware/v2r2/blob/feature/vr-fsm/tests/utils/fuzzer.rs#L14-L30). If a given
shrunken history fails the test it is once again shrunk to attempt to find the minimal failing
history. The block search is not exhaustive as that is prohibitively expensive. It begins by
removing the first half of the history and re-running the test. Then it tries the second half of the
history (always leaving the last message, since that caused the failure). If the test does not fail,
it removes the first quarter, then the second quarter, etc... until it gets down to a blocks size of
1 . Note that this is not exhaustive because it doesn't remove say, messages 2 and 3, from a history
of four, but only 1+2, and 3. It also will not attempt to remove disjoint messages when the block
size is greater than 1, for example removing messages 1 and 3. Becoming exhaustive would require
testing `n!` histories and is simply not feasible. The strategy is therefore optimistic in that it
hopes to remove large blocks of co-located messages from the history early on so that the number of
histories tested is vastly reduced. When all large chunk sizes are exhausted and there still remains
a history greater than 1 message, the shrinker will exhaustively start removing messages 1 by 1 and
re-running the history. This is `O(n^2)`, but required to find the minimal history. If we are lucky,
by the time we get to this step the history has already been substrantially shrunk.

Note, that if we have time it would be amazing to work on a partial order reduction strategy instead
of the block reduction strategy to remove known unrelated messages. This is probably just an
academic exercise however, as the current strategy is fast enough and has already found significant
bugs in practice while returning the minimal history. Note also that dynamic partial order
reductionas requires domain knowledge of the messages, and thus further test callbacks to aid in the
shrinking.
