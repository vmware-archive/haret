# The State of V2R2 Address

The State of V2R2 is strong. Finite state machines, generative testing, non-blocking channels,
static typing, and memory safety have provided an excellent foundation on which to build a reliable,
strongly consistent distributed data store. Tonight, my fellow VMwarians, we shall look back at what
has been accompilshed these last nine months and share glimpses into the future of the project.
The road is not always easy, as we all must deal with the dreadful realities of asynchrony and
partial failure. We must endure the hard work of fighting the borrow checker, the inflexibility of
typed channels, and the temptation to use `unsafe` code. While we may question our lot at times, may
we all find peace as we learn to embrace these tools.

# The Building Blocks

V2R2 is written in the [Rust Programming Language](https://www.rust-lang.org/). Rust is a strongly
typed, modern, systems programming language that provides for memory safety and the elimination of
data races at compile time. The benefits of Rust are outside the scope of this document, but a good
description can be found [here](http://www.oreilly.com/programming/free/files/why-rust.pdf).

The goal of V2R2 is stated in the README:
> V2R2 is software for providing a distributed service built on proven protocols that exposes
> strongly consistent coordination primitives via an easy to use API, in order to satisfy the
> essential dependencies of many distributed applications.

V2R2 implements the Viewstamped Replication protocol as described in [Viewstamped Replication
Revisited](http://pmg.csail.mit.edu/papers/vr-revisited.pdf). It uses this protocol to provide
linearizability guarantees for operations against a replicated, hierarchical data store. The system allows
multiple independent hierarchical data stores on a shared cluster of hardware. Each data store is
backed by a replica group (tenant) that utilizes the VR protocol to provide replication. Data stores
are independent of each other such that linearizability is only guaranteed across a single data
store. This allows for potential parallelization of operations and more efficiency on a partitioned workload,
but prevents things such as cross store transactions.

V2R2 as currently written, is built around 3 main building blocks:
  * Finite State Machines (FSMs)
  * Non-Blocking Channels for communication between threads
  * Event loops for handling network communication and timers

Finite state machines in V2R2 are modeled after the
[`gen_fsm`](http://erlang.org/doc/design_principles/fsm.html) behavior in Erlang. They are a set of
state functions, where each function represents a specific state of the fsm, along with an
associated context that maintains the internal state of the fsm.  They are part of the [Fsm
library](https://github.com/andrewjstone/fsm) written for use in this project. The primary use case
for FSMs is to implement replicas in a replication group. A replica FSM is addressable via a `(name, node)`
tuple, exists in a given state at any time, and is started, stopped, and communicated with
via an intermediary called a `dispatcher`.  Currently all FSMs are run in the same thread as the
dispatcher. This is not a requirement, and it is expected that FSMs will be run in a thread pool
at a later stage in development.  When a message arrives at the dispatcher destined for an FSM,
the function that represents its current state is called with the current context and the message.
The function returns the next state and any outgoing messages to the dispatcher, which will then
save the state and forward the messages as needed.

The dispatcher itself is basically just a router that manages access to any FSM on it's Node (OS
Process), along with a hashmap to store repica state. It has knowledge of the cluster and can
forward messages to other nodes over TCP, via an event loop backed by
[mio](https://github.com/carllerche/mio). Cluster setup itself will be described in the next
section.

In V2R2 there are other components running in their own threads, such as TCP servers that handle the
admin and client APIs. Seperate threads require a way to communicate in a memory safe manner. In
V2R2 we utilize [Rust Channels](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) for this
purpose.  Rust channels are strongly typed multi-producer, single consumer (mpsc) channels that are
non-blocking and atomic. They are very well suited to an asynchrounous protocol such as VR, although
the strong typedness makes them cumbersome to abstract and use generically.

As mentioned above, all network communication is performed via servers utilizing `mio` event loops. There are four
such event loops (bound to four different TCP ports) that implement different interfaces in the system:

 * Admin API - Management commands for joining nodes in a cluster, creating tenants, and inspecting
   the system
 * Client API - The use of the datastore in a given tenant for linearizable operations
 * Cluster API - Internode communication needed for gossip and tenant setup
 * Peer API - Connections between nodes that allow transport of VR protocol messages

All communication over TCP is done via [Msgpack](http://msgpack.org/index.html) encoded data
structures in frames with a 4-byte size header. The [`ClientEnvelope and
ClientReplyEnvelope`](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/envelope.rs#L38-L60) are
examples of encodable data structures that are sent between the [API
client](https://github.com/vmware/v2r2/tree/9112f7f0749424714976067e5ef6094f195fce63/src/bin/v2r2-cli-client.rs) and [API
server](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L185-L198).

# Operations and Cluster management

A correct, well tested system is useless if no one can figure out how to operate it. Operation
should be as simple and intuitive as possible. In order to facilitate this ease of use we must
clearly define the entities that exist and how they compose. There are only a few such entities that
we are concerned with:

 * Node - An OS process on a Host with an IP address on the cluster network, and a unique name
 * Cluster - A set of Nodes connected via `Join` operations.
 * Replica - A peer participating in the VR protocol of a given tenant with a unique name per node
 * Tenant - A group of replicas managing a datastore

Note that a replica can only exist in one tenant, and replicas don't talk to replicas in other
tenants.

We can start to build up a working system by configuring and starting a single node. We must give
the node a name and assign its ip addresses with a [config
file](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/config.json). We can then start up other configured
nodes and `join` them together via the [Admin
CLI](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/bin/v2r2-admin.rs). Once we [create a
tenant](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L252),
we can [discover the name of the primary from the admin
cli](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/bin/v2r2-admin.rs#L248). We can then connect the
API client CLI to the node on which the primary exists and start issuing operations against the
newly created tenant.

For ease of development and getting started, a new user can generate a development cluster with `make devrel`. The
cluster will contain nodes `dev1..devN`, all assigned unique ip ports on localhost. A brief tutorial
on getting started can be found in the
[`README`](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/README.md#setting-up-a-test-cluster)


# The Client API

There is only a single client currently, and it is an interactive CLI client. A user can issue CRUD
operations against a hierarchical datastore consisting of typed elements. The root of the tree is
`/`, and it utilizes slash separated paths for element names similar to a filesystem. Nodes can only be created if
their parent exists in the tree. The [client
help](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/bin/v2r2-cli-client.rs#L320-L347) shows the
full set of commands that can be issued against the tree.

Note that in it's current form, while you can attempt to create elements of different types, they are all
treated by the code as binaries, as the type specific code does not yet exist.

# VR Protocol implementation

Replicas in the VR protocol are implemented as named
[`Fsm`s](https://github.com/andrewjstone/fsm/blob/9112f7f0749424714976067e5ef6094f195fce63/src/fsm.rs#L43-L47) that communicate by
sending messages to other named FSMs via the dispatcher. FSMs themselves are not running in their
own thread of control, and therefore cannot directly initiate sending messages. All messages sent
are a result of the dispatchaer calling
[`fsm.send(msg)`](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L342-L347) for the
given replica. In order to drive activity in these FSMs when no external activity is occuring, the
FSMs are called with periodic
[`Tick`](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L624-L629) messages. This
allows the FSMs to check for any expired timers and return outgoing messages.

The implementation of the VR protocol is complete as described in the paper. It is tested via generative
testing and basic operational testing for normal, view change, and recovery modes of operation.
Reconfiguration is implemented (with one exception discussed below), but is only minimally tested.
Generative testing, based on the framework described below, should be more heavily utilized to
test the entire protocol together with reconfiguration.

As written, the paper does not consider the case of long lasting partitions during reconfiguration.
Although likely rare, this lack of support results in a cluster unable to utilize certain nodes
permanently, which reduces the number of nodes that can now fail before the cluster is permanently
offline. The full scenario is described in [other
notes](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/docs/vr-made-live-notes.md#long-lasting-partitions-and-reconfiguration),
along with a planned solution. Once implemented, the VR protocol itself will be complete. Of course
more testing and benchmarking will be required.

There has been little attempt at protocol or systems optimization yet, as the main goal has been to
get the code working first. No optimzations from either sections `5.1` or `6` in the paper have been
implmented.

# Tenants
Each replica group in the cluster is called a Tenant. These tenants are managed as a [mergable data
structure](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/tenants.rs#L6-L10). A new tenant is
created by an admin client sending a `CreateTenant` message which is then received by the admin
server on a single node and forwarded to the dispatcher on the same node. When a [tenant is
created](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/tenants.rs#L6-L10), it's given a UUID,
and inserted into the mergable structure. During a [management
tick](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L394-L410) the set of tenants
is [sent to all other
nodes](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L394-L410) in the cluster and
[merged with the local
state](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L637-L656). This allows nodes
to learn of, and start new local replicas.

Additionally, when a reconfiguration is committed by the primary in a tenant, a message is sent to
the dispatcher with the new configuration, and the tenants structure is [safely
updated](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/src/vr/dispatcher.rs#L637-L656). This is safe
because the reconfiguration was committed by the VR Protocol, and the changes are only reflected if
they have a later version number. These updates will reach the other node when they get gossiped
around in the management tick.

Note also, that tenant creation and reconfiguration only work with replicas on nodes
participating in the cluster.  If an admin client tries to create a tenant with replicas existing on
nodes not known to be in the cluster by the local node, the request will fail. We plan a similar
feature to prevent removing nodes from the cluster while they have active replicas running. However,
this has not been implemented yet.

# Testing
The bulk of the testing has been done by a generative testing tool similar to ``eqc_statem``, a
stateful testing component of [`Erlang
QuickCheck`](http://www.quviq.com/products/erlang-quickcheck/). Although fantastic for Erlang
projects, EQC itself is closed source, with little public documentation. My tool cribbs ideas from
it, although it is of much less general use and geared specifically to testing the VR protocol in Rust
utilizing the FSM model. The most current, and thorough, documentation is
[here](https://github.com/vmware/v2r2/blob/9112f7f0749424714976067e5ef6094f195fce63/docs/initial-vr-fsm-architecture-and-commit-msg.md#testing).

So far, basic VR operation, recovery, and view change are very well tested on a connected network
that can experience crashes and restarts of nodes. There remains much more testing to be done,
including dropping and delaying messages, and forcing partitions. These additional failure modes
must be tested during reconfiguration as well. The testing framework is well suited to this task.

# Closing Remarks

I hope I have clearly revealed what exists at this state of our project's existence. There remains
much to be done, and not just reconfiguration, testing, benchmarking and documentation. There is
much to be abstracted out into separate libraries, such as cluster management, msg dispatching, and admin
configuration. We need to fully implement datastore element types and a create or use a persistent, snapshottable
backend. We must refine the configuration process, and enhance it with a planning and commit stage.
In order to do this, we require consistently tracking those plans inside the cluster. We therefore
plan to create a root replica group (tenant) to allow safe, stateful planning and transitions.
Beyond that, there are [Zookeeper style
watches](http://zookeeper.apache.org/doc/trunk/zookeeperProgrammers.html#sc_zkDataMode_watches) and
other high level distributed primitives to implement.

There is tons to do, but we shall remain committed I am very excited to get to continue on this
journey with all of you.
