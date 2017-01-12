# Introduction
haret is a strongly consistent coordination service that provides a high level API for distributed
systems management and metadata. haret utilizes the [Viewstamped
Replication](http://pmg.csail.mit.edu/papers/vr-revisited.pdf) protocol to provide [strict
serializability](http://www.bailis.org/blog/linearizability-versus-serializability/) of operations
over one or more hierarchical data stores replicated across multiple nodes.

A haret cluster consists of multiple **nodes** (typically one per physical or virtual machine) that are connected via TCP
to provide a clustered, highly available and fault tolerant service. Each cluster consists of one or
more **namespaces** containing a **hierarchical data store**, that can be operated independent
of and concurrently with other namespaces on the same cluster. Each namespace utilizes its own
group of participants known as **replicas** that implement the Viewstamped Replication protocol and
allow safe operation over that namespace's data store. Nodes, Namespaces, and Replicas are
created and configured via an administrative API which allows dynamic membership changes to both the
cluster and namespace replica groups at runtime.

The data store in each namespace consists of a tree where only the leaf nodes contain data.
Leaf nodes have a type set at creation time that establishes their behavior and API. Leaf
nodes can be of type `blob`, `queue`, or `set`. Interior nodes are always of type `directory`. Each
data store is operated on over a client API that ties each connection to a single namespace.
Operations cannot cross namespaces.

In addition to operations on individual nodes in the data store, transactions over multiple
nodes are also supported. High level primitives such as mutexes, semaphores, reader/writer Locks,
barriers, and leader election will be provided directly by haret for application-level coordination,
although they have not been implemented yet.  Finally, client subscriptions to updates on subtrees
of a given namespace are planned to allow piping of information to secondary external systems.

The remainder of this document contains further information about the primitives mentioned
in this introduction, as well as descriptions and examples of the APIs available to administrators
and clients of haret.

# Ports, Message Format, and Serialization
haret provides administrative and client APIs over TCP connections. Nodes connect with each other
over TCP connections seperate from both administrative and client connections. The listening hosts
and ports for each of these interfaces is configured in
 [config.json](https://github.com/vmware/haret/blob/master/config.json).
 * `cluster_host` - cluster server for internode communication
 * `vr_api_host` - API server
 * `admin host` - Admin server

The admin and cluster interfaces utilize [MsgPack](http://msgpack.org/) encoded data and do not
contain public APIs. Admin interaction is performed via the CLI admin client which implements the client
side of the protocol. For future interoperability, it's possible this interface will become an http
or protobuf interface instead. In that case it would be fully documented.

The API interface is a public interface that is expected to be implemented by haret clients in other
languages. All messages are [protobuf](https://developers.google.com/protocol-buffers/) encoded and
framed with a 4 byte length header in network byte order (big endian). The complete set of messages
is defined [here](https://github.com/vmware/haret/blob/master/src/api/messages.proto).

Currently, all that is implemented is the base K/V API so all messages are either requests or
responses wrapped in an
[`ApiMsg`](https://github.com/vmware/haret/blob/master/src/api/messages.proto#L3-L8). Asynchrounous
notification messages used by the subscription subsystem will be added later along with higher level
coordination primitives.

A more detailed client implementation guide is forthcoming.

# Cluster Administration
Each haret node is started with a configuration consisting of its name, its cluster IP address and
port, its admin server IP address and port, and its client API server IP address and port. The
cluster server IP address is what enables haret nodes to connect to each other and pass messages. All
cluster server IPs for communicating nodes must be on the same network so that they are reachable
via TCP.

The first step in operating a cluster is to join nodes via the admin CLI using the `cluster join
<Node>` command in the admin cli. This sends a request to the node that the admin CLI is connected
to and instructs it to begin the join process. The server returns `Ok` when the request is
receieved, not when the cluster is joined. Joined cluster nodes exchange membership information
backed by an [Observed-Remove Set](https://github.com/aphyr/meangirls#or-set). Each node then
connects to every other node in the membership set to form a full mesh. Because of the gossip nature
of this process, if the initial node receiving the join request dies before it forwards
the request, the join request may be lost. There are some thoughts around making joins synchronous to some
extent to prevent this problem.

Once nodes are joined together, the user is likely to want to create a namespace to make the cluster
useful. This can be accomplished with the `vr create namespace ...` command. The following command
creates a namespace named `test-ns` with 3 replicas. Each replica has a name of `r` and runs on
either node `dev1`, `dev2`, or `dev3` which resides on localhost with a unique port. The node
receiving the request will start up any replicas it hosts and then forward the request to other
nodes so they can start their replicas. Once the replicas are up they will elect a leader via the
viewstamped replication protocol.

```
vr create namespace test-ns r::dev1@127.0.0.1:2000,r::dev2@127.0.0.1:3000,r::dev3@127.0.0.1:4000
```

Note once again, that this command is not synchronous or strongly consistent. Two administrators
could connect to different nodes and create the same namespace with different member replicas
concurrently. Currently the onus is on administrators to prevent this type of confusion, but in the
future it is likely to be fixed via either a root consensus group used for strongly consistent
operations that runs on every cluster or 2 phase commit via all nodes. Other solutions are welcome.

The rest of the admin CLI commands can be discovered via running the admin client:

 ```$ target/debug/haret-admin 127.0.0.1:2001 -e -h```

Once a consensus group is up and running, it can be utilized via an API client. Currently, the only
API client that exists is the CLI client.


# Client Operations
Each client operates on a single namespace. A client connects over TCP to a single node in the
cluster and identifies the namespace it wants to operate on with a
[`RegisterClient`](https://github.com/vmware/haret/blob/master/src/api/messages.proto#L18) request. The node
will respond with the current primary for that namespace if it is on that node. Otherwise, it will
redirect the client to the node containing the primary. Alternately, the node may instruct the
client to retry later or respond that the namespace does not exist.

Once connected, a client may issue requests to the current primary. These requests will be
described in the following subsections. Note that all requests are not enumerated here. These
sections are intended to give a broad overview of current capability and plans for the future, not
serve as detailed instructions for using haret, or building a client. Either this document, or
another will be fully fleshed out in the future to provide a complete guide to the Client API.

### Data structure operations
Each namespace consists of a hierarchical collection of tree nodes. Each node is identified by a
UTF-8 `path`, similar to a filesystem path with the `"/"` signifiying levels within that path.
Only leaf nodes contain data, and each leaf node has a type such as `queue`, `set`, or `blob`. The
type of the leaf node is determined when it is created, and cannot be changed. Each type has a
unique set of operations that can be performed upon it. These operations can be performed one at a
time, or bunched together into a `Multi-CAS` transaction which allows conditional execution based on
version checks at existing nodes. Note that all operations pass through the consensus mechanism, and
end up as an entry in the VR Log. A Multi-cas operation, like a regular op, results in one log entry
as it is executed atomically in the upcall to the backend tree code.

As of right now, the only existing API client is the [Interactive CLI
client](https://github.com/vmware/haret/blob/master/src/bin/haret-cli-client.rs). While this client is
useful for testing and debugging, it does not implement the entire client API, even as currently
defined in the protobuf file. In fact, it's unlikely it ever will implement the entire API as it's
syntactically difficult to map some operations into a CLI string. For now though, it's a good way
for users to become acquainted with Haret.

Once a namespace has been created via the admin client, it needs to be `Entered` from the CLI
client so it can be operated on. Entering a namespace is what causes a `RegisterClient` command to
be sent to the connected Node and start the process of discovering the primary for a given namespace. Upon success, a user should see something like the following:

```
haret> list-namespaces
test-ns

haret> enter test-ns
Client registered. Primary = name: "r" group: "test-ns" node_name: "dev1" node_addr:
"127.0.0.1:2000"
```

Once the namespaces is entered, other tree-related operations can be performed as shown below.

```
haret> create set /some/set
Ok
Epoch = 1, View = 978, Client Request Num = 2
haret> create set /some/other/set
Ok
Epoch = 1, View = 978, Client Request Num = 3
haret> set insert /some/set blah
true
Version = 1 Epoch = 1, View = 978, Client Request Num = 4
haret> set insert /some/set hello
true
Version = 2 Epoch = 1, View = 978, Client Request Num = 5
haret> set union /some/set /some/other/set
hello
blah
Epoch = 1, View = 978, Client Request Num = 6
```

### Multi-cas Transactions
haret transactions don't use a specific transaction language, or
`Begin..End` statements. They do however allow multiple conditional operations to run atomically and
in isolation from other transactions.

A transaction consists of multiple operations that are run iff the guard conditions in the
transaction are all true. Guard conditions are simply version numbers on a set of keys that must
match the current version numbers in the tree at runtime. Conditional operations are only checked
against versions and not the data in the tree. If any of the guard versions do not match the
versions at the nodes in the tree, then the transaction fails. On success the return value for each
operation is returned in the order of submission.

Note that transactions are implemented in haret and exist in the protobuf definition, but are not
implemented in the CLI client.

### Subscriptions
Clients often want to be notified of any changes to a particular subtree. In order to prevent
unneccesary queries to achieve this goal, haret plans to provide a notification system. A client can
subscribe to all changes against a given subtree via a key `prefix`. Any committed operation against
a node that matches the key prefix will be sent as a notification to a registered client. It should
be noted that there are no recency guarantees made about notifications. Multiple updates can occur
between the time between when a notification is sent and received. The only guarantee is that while
a client is connected and subscribed it will receive all notifications for committed operations in
order of commit.

The exact serialized format of the operation sent by a client is what will be sent in the
notification. Optional flags on the initial subscription determine whether the values of the command and result are sent to the subscriber, or if only the identifying command itself is sent. For transactions,
the entire serialized transaction operation is sent upon commit if any of the operations in the
executed transaction contains a matching prefix.

To allow load balancing, subscribers can be attached to any node in the cluster containing a replica
participating in the consensus group/namespace of the client. This is allowed due to the fact that
no recency guarantees are made about notifications.

There are two types of subscriptions: `oneshot` and `persistent`. A `oneshot` subscription only
sends the next notification for any matching commit on the registered prefix, then removes the
subscription. A `persistent` subscription will send notifications for all commits matching the
prefix until the subscription is explicitly cancelled or the client is disconnected.

### Coordination Primitives

haret is not just a storage system for metadata. More fundamentally, haret exists to make managing
distributed systems infrastructure easier and less error prone. With this goal in mind haret
provides a number of coordination primitives "out of the box". There is no need to use 3rd party
libraries, or build such primitives yourself in an ad-hoc manner. Leader election, locks, barriers,
semaphores: You got it!

**Important Notice** - These primitives have been thought through and abstractly designed, but are not
yet implemented.

The following primitives are planned for inclusion in haret. Further detail will be provided upon
implementation.

* Leader Election
* Locks (Re-entrant and Non-re-entrant)
* Semaphores
* Barriers, Double Barriers

### Limitations
For now there are no built in limitations around node size. However, for performance reasons,
indivdiual nodes (including directories) should not exceed 1 MB. The system will provide tools to
help monitor node size and performance statistics.
