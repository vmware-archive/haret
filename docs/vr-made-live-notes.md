# Things needed to make VR into a real system

## Bootstrapping: Replica group initialization and replica startup
The VRR paper mentions that replica groups start off in epoch 0 with an empty config and require an
initial view change to transition to a group with at least 3 members. However, there is no
discussion of how to bootstrap the system and force an initial election. How do the initial replicas
learn of each other?

In our implementation we have solved this bootstrapping issue along with the capacity for multiple
tenants by separating the concepts of replicas from nodes. A replica is a participant in the
Viewstamped Replication protocol, while a node is an OS process running on a given physical host
that contains running replicas. Each node contains a module called the `Dispatcher` that has 3
primary roles:

 * Management and initial configuration of tenants
 * Startup of replicas in a given tenant
 * Name service lookup and routing for VR messages

A replica group is created with a call to `dispatcher.create_tenant()`. This call takes the initial
members of the group and generates a tenant id (UUID) for the group. If the members are all on the
local node the dispatcher will start them up and pass them in to the replica constructors as the
`new_config`. They will then proceed to elect a primary via the normal view change protocol. Note
that this local group setup is very useful for testing, but not so useful in production where
replicas should be failure independent. If however, some replicas exist on different nodes, the
dispatcher will gossip the configuration around to other nodes using the cluster membership
information. The remote replicas will be started when their local dispatchers learn of the new
tenant.

Similarly, during reconfiguration new replicas may be added to a replica group (tenant). The old
configuration will commit this to the log and via the reconfiguration protocol attempt to transition
to the new group. However, there needs to be some way to start the replicas in the new group. They
are started in the same manner as during bootstrapping: namely, the configuration is gossiped around
and the dispatcher on each node will start it's local replicas and pass in the configuration.

Note that the mechanism above can result in conflicts if two different configurations are gossiped
around. Since all reconfiguration is performed through the VRR protocol, we know definitively that one
configuration is later than the other since it has a higher epoch. We gossip around the epochs with
the configuration so that we can successfully converge.


## Long lasting partitions and Reconfiguration
The VRR paper assumes that a replica in the new group always has the old and new configurations when
it learns of a later epoch. It is then supposed to catch up by sending state transfer messages to
replicas in the old group. However, it is possible that a replica can be partitioned off through
multiple reconfigurations and be the only surviving replica in a later configuration. In this case,
when it learns of a new epoch, perhaps from a normal prepare message from the primary, it will not
be able to commence state transfer since it does not know who to ask for the state. Note also, that
if the replica learns of a later epoch in normal mode it's likely the replicas in the latest old
config have shut down because they have received a quorum of `EpochStarted` messages. In this case
the replica will need to transfer state from a replica in the new group.

The protocol as written does not provide for this scneario, although it is easy enough to work around in
practice. There are two methods of obtaining the old and new config in this scenario and performing
state transfer. The replica can directly ask the replica that informed it of the latest epoch. In
the normal case this is the primary as a result of a `Prepare` message. During View Change this
would be a backup replica. Both of these are safe in that the replica that was partitioned off will
learn of operations later than any that it has participated in the commit of, and so will not
participate with forgotten information.

The second method is specific to our implementation, which is based on gossip for starting replicas.
The replica can ask the dispatcher that participates in gossip for the latest config for the given
tenant. If the local dispatcher has not learned this information yet it can broadcast a request to
other nodes, or simply wait for the state to converge before replying to the local replica.

## Efficiency
There are a number of major and minor adjustments we can make that make the VR implementation more efficient. Some of them are described in the paper, and some have been discovered by us during implementation and testing. This section will cover optimizations made in our implementation.

#### Primary Crash and Recovery
When a replica crashes and restarts it enters recovery mode. This results in a `Recovery` message being sent to the other replicas when it restarts. If, however, in a given view, the primary crashes and restarts before the backups notice the primary crashed, the backups may receive a Recovery message from the crashed primary, which they currently believe is the active primary. In this instance, as described in the paper, the backups are still in 'normal' status and should respond to the recovery message. However, the primary will never recover in this view because it itself is the primary for the largest view received by the quorum of `RecoveryResponse` messages. 

Note that the above scenario is safe because eventually the backups will timeout and start a view change and one of them will become a new primary and respond to further Recovery messages. Therefore the old primary will eventually recover. However, it is suboptimal, in that it requires waiting for a timeout to trigger a view change that can be detected without a timeout. We can eliminate the remaining view change timeout wait time by having a backup start a view change immediately if it sees a `Recovery` message from the primary of the current view. Additionally, the backup does not respond to this message.
