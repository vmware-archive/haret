Nodes Joining and Leaving a haret cluster
=================

(This is a very rough draft, for purposes of comment and discussion, and
should not yet be treated as canonical.)

From a membership point of view, nodes can be in one of three states:

- **surplus:**
A surplus node is known to the rest of the cluster (by IP, port, and Node
ID) but is not currently participating as a member of any replication groups.

- **participating:**
A participating node is a member of at least one replication group, and
thus its availability affects the user-facing availability of the cluster.

- **ejected:**
An ejected node was formerly participating but has been removed from all
replication groups, possibly due to host or network failure.

(note: The "recovery" state within VR is still considered to be participating
for the purpose of membership. A backup node within a replication group might
crash, restart, and go into recovery mode, but would still be considered a
participating node if no other action was taken.)

Nodes only go through these states in order. That is, the lifecycle of any
given Node ID is always:

**unknown -> surplus -> participating -> ejected -> unknown**

Each transition would occur as follows:

- **unknown -> surplus**

On an individual node, one uses the command line to supply the IP/port of one
or more currently not-unknown nodes. Through (possibly eventually-consistent)
group communication, the node will become known to the others. Once a node
knows of any other nodes, it enters the surplus state.

- **surplus -> participating**

An administrator can issue one of two commands that cause this transition.
Either they can explicitly request that a specific node be added to a specific
replication group, or they can request that the entire cluster automatically
determine the needed view changes in order to satisfy desired constraints
such as having enough participating nodes in every group to satisfy the
preferred failure tolerance. There are two reasons why the more specific first
command might be used: either to create a new replication group via its
initial member, or to overprovision a replication group in preparation for a
planned removal of another node. No matter which of these commands is used,
the disruptive nature of this transition (due to forcing view changes) means
that it will require two steps on the part of the human. The first step
produces a plan, showing the user what changes will be made to membership and
what replication groups will undergo view changes. The second step is a
"commit" step, actually causing those events to occur.

We may also choose to allow a node to move along this transition without an
administrator action in certain circumstances. Failure-driven reconfigurations
(ejecting a failed leader) are permitted to move a node on the
surplus -> participating transition if such a surplus node exists. This would
allow an administrator to provide a cluster with a warm spare. This would mean
that in the case of a failure, replication groups could use the presence of
such a spare to self-heal.

- **participating -> ejected**

Much like the preceding entry, there are two ways that a node can go through
this transition. In the case of a failure-driven view change, a replication
group may remove a node from that group. 

Also, an administrator may issue a command to force a view change that will
remove a given node from a given replication group. ("forced ejection")
Similarly to the above, as this command may cause user-visible availability
issues and also may move a replication group below the targeted size for its
desired failure tolerance, this command will be a two-stage (plan, then
commit) procedure.

Whenever a node is no longer in any replication groups, regardless of whether
it was ejected from those groups forcefully or manually, it considers its
membership state to change to ejected.

- **ejected -> unknown**
 
This transition is purely for cleanup purposes, to remove unecessary metadata
about past nodes from persisting permanently. An administrator can issue a
command to move any or all ejected nodes out of the known set, deleting them
from the data structure used to track all known nodes. This is manual as the
amount of data being consumed is small and may be very useful for debugging,
in order to (e.g.) see recently ejected nodes at a glance.

Open Questions
=============

How should membership communication happen across the cluster?
I am currently imagining a CRDT stored per node that stores the set of nodes
and their membership states, using a non-transactional communication such
as some sort of gossip between them. An initial version of gossip could be
a full communication between all nodes, as active cluster sizes at least early
on are expected to be less than 10 nodes.

A related question is how to display that information to an administrator,
especially as it is possible that it may not be 100% consistent. One thought
is that the command to view membership might have two variants, chosen by a
command line flag. One of them would be "free" from a network cost point of
view and would just show the local host's copy of the data. The other would
request the data from all hosts, merge the CRDTs if they differ, and show the
resulting set. It would also, in the interest of convergence, push that merged
data back to any hosts that sent it anything other than the final view. 

Should an ejected node be able to move to surplus or participating, instead of
only having the choice of being removed from membership? The arguments in
favor of not allowing that mainly come down to simplicity. The linear lifecycle
is easy for an operator to reason about and easy for us to verify the
correctness of. Should one wish to return an ejected node to service, they
could simply restart and rejoin it, which would place it into surplus status.
If we allow it to keep its log and snapshots (but require it to generate a new
Node ID) across this process, it should be able to re-enter service quickly.


