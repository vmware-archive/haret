 * Start Date: 2017-05-15
 * Haret Issues: https://github.com/vmware/haret/issues/93
 * References: [Viewstamped Replication Revisited](http://pmg.csail.mit.edu/papers/vr-revisited.pdf)

# Summary

In normal mode of *Viewstamped Replication*, when the primary fails or gets partitioned from backups
the backups will recognize they have not received a message within a given timeout and attempt to
elect a new primary via the **View Change** protocol, as described in section 4.2 of the VRR paper.

The protocol consists of three messages:

 * `StartViewChange` - A replica broadcasts this after it times out or receives this message
 * `DoViewChange` - When a replica receives a quorum of StartViewChange messages it sends this
   message to the proposed primary (which is chosen in a round robin manner starting in view 1)
 * `StartView` - When the proposed primary receives a quorum of DoViewChange messages it sends this
   message to all other replicas to instruct them to exit view change and become backups in the new
   view.

This protocol is safe, but is very expensive since it requires each replica to send its complete log
to the proposed primary in the `DoViewChange` message, and requires the new primary to send its
entire log to each replica in the `StartView` message.

The goal of this RFC is to modify the view change protocol in the simplest manner possible to
minimize the amount of data transferred during view changes. No new messages should be introduced,
and overhead to existing messages should be as small as possible.

# Motivation

It is terribly wasteful to send the entire log during `DoViewChange` and `StartView` messages. This
RFC will provide an optimization to minimize the amount of state transferred between replicas during
view change and minimize view change latency.

# Detailed Design

In order to minimize state transfer during view change we need a single specific piece of
information that can be shared among all replicas: What is the last log entry for each replica?

For this sketch, we will ignore epochs and reconfiguration as there is no change to the protocol
with regards to them.

If we were trying to ship the absolute minimum amount of state during view change, we'd need each
replica to keep track of the following information for each other replica:

 * Last normal view
 * Op num
 * Commit num (log index *<=* op num)

We would need to maintain the last normal view because when sending `StartView` messages, the new
primary could check if the last normal view of a replica was the latest last normal view and
therefore only have to send missing entries from that view, beginning with the op num of the replica
up to the max op num of all the replicas in the `DoViewChange` quorum. If the last normal view of a
replica was not the latest last normal view (as determined by the `DoViewChange` quorum), the new
primary would have to send all messages since the last commit of that replica as messages could have
been reordered during previous view changes. This is the same protocol used in state transfer as
described in section 5.2 of the VRR paper. The same type of analysis would be done when determining
what log entries each replica would send to the primary in `DoViewChange` messages.

Doing things this way minimizes the amount of state shipped during view change but adds significant
overhead to the protocol operating in *normal* mode. Every `Prepare` message from the primary now
has to contain the known `op`, `commit_num` and `last_normal_view` of **all** replicas (minus the
replica being sent to). For a 5 replica cluster, this is an overhead of 96 bytes on every `Prepare`.
If prepares aren't waiting, the new information learned at the primary from `PrepareOk` messages
will be sent to the backups with the next Commit message, rather than piggybacked on the next
`Prepare`.

This is a good solution in that it mimizes the amount of data sent during view change, but it still
has a lot of overhead, especially for a large number of writes with small sizes. By slightly
increasing the amount of data transferred during view change, we can minimize the amount of overhead
in the normal protocol. Since it's expected in a healthy cluster that all replicas are almost
always up to date, we can instead just keep track of the commit number of each replica at the
primary. We can then send the minimum of the commit nums of all the replicas in each `Prepare`, only
requiring 8 bytes of overhead. During view change, `DoViewChange` and `StartView` messages will send
only log messages that come after the minimum commit entry of all the replicas.

If a replica starts to lag too far behind, the primary can raise an alert. If multiple replicas lag
too far behind, in addition to raising alerts, the primary can stop accepting new requests until
they catch up, as a form of back pressure. Furthermore, this global commit knowledge also provides a
frontier from which to enable garbage collection of the log.

# Concrete changes to the state machine

1. PROTOCOL: The global minimum commit `m` is added to the prepare and commit messages sent by the primary.
2. PRIMARY: The primary will maintain a mapping of the lowest seen commits for each replica in the quorum.
3. REPLICAS: All of the replicas will maintain a state variable containing the global minimum commit detailed in the last prepare or commit message.
4. PRIMARY: When the primary prepares a commit or a prepare message to be sent it must first update its own entry in the mapping, then take the minimum number in the mapping to add it to message to be sent.
5. PRIMARY: When the primary receives a `PrepareOK` message from a replica, it can assume that the operation `o` less one is the minimum number that the replica has committed, as the protocol requires that all `Prepare` messages be processed in order.  It then updates its commit mapping for the sending replica to `o - 1`.
6. REPLICAS: When a set of replicas starts a view change, their `DoViewChange` message should contain the tail of the log from the global minimum commit `m`, instead of the entire log.  The new primary then select the log tail as per the protocol.
7. REPLICAS: When the new primary sends the `StartView` message, it only sends the log tail from `m` instead of reshipping the entire log.
6. REPLICAS: When a set of replicas completes a view change, the new primary will initialize its mapping with its last known global minimium.
7. REPLICAS: When a set of replicas completes a epoch change, the new primary will initialize its mapping with the operation number of the reconfiguration request.

# Advantages

This scheme drastically reduces the amount of data transferred during view changes. It is also
extremely simple and adds a minimum amount of overhead to normal `Prepare` messages: just 8 bytes to
track the minimum commit num at all replicas. The additional tracking of the commit nums at all
replicas by the primary also allows threshold based alerting for slow replicas, and potentially
provides a means for backpressure. It also allows a global frontier for log garbage collection.

# Drawbacks

There aren't really any drawbacks.

# Alternatives

Section 5.3 of the VRR paper discusses an alternative where `DoViewChange` messages only send the
last log entry. If the proposed primary only requires that entry to become up to date, it then
proceeds to send `StartView` messages. Otherwise, it must enter state transfer. This scheme has 3
drawbacks:

 1. State transfer isn't supposed to occur in view change mode as described in Section 5.2 of the VRR
    paper.
 2. State transfer requires extra messages in the case that the proposed primary is not up
    to date.
 3. This optimization provides no changes to the `StartView` message, which still ships the entire
    log from the new primary to the backups.

Another optimization was attempted in [RFC
1](https://github.com/vmware/haret/blob/master/rfcs/rejected/1-view-change-optimizations.md) but
rejected in favor of this proposal as this proposal is much simpler and safer.
