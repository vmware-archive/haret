 * Start Date: 2017-05-10
 * Haret issues: https://github.com/vmware/haret/issues/93

# Summary

When the primary fails or gets partitioned from other replicas, the other replicas will notice via
timeout and attempt to elect a new primary. This is known as a **View Change** in the *Viewstamped
Replicaton Revisitied (VRR)* protocol. The view change protocol consists of 3 messages:

 * `StartViewChange` - Instruct all other replicas that a view Change is occurring
 * `DoViewChange` - When a replica receives a quorum of StartViewChange messages it sends this message to the proposed primary (which is chosen in a round robin manner starting in view 1)
 * `StartView` - When the proposed primary receives a quorum of DoViewChange messages it sends this message to all other replicas to instruct them to exit view change and become backups in the new view.

 This protocol is safe, but is very expensive since it requires each replica to send its complete log to the proposed primary in the `DoViewChange` message, and requires the new primary to send its entire log to each replica in the `StartView` message.

The goal of this RFC is to devise a view change protocol that minimizes the amount of data transferred during view changes, while maintaining the safety and liveness guarantees of the current protocol.

# Motivation
It is terribly wasteful to send the entire log multiple times during a view change. Ideally the primary being elected would already have the most up to date log, and would only need to send the missing log entries to each new backup.

# Detailed Design

The most significant change to the view change protocol is that there will be no state transfer to the new primary. Instead a new message, `BecomePrimary`, will be introduced after the round robined proposed primary receives a quorum of `DoViewChange` messages. The `DoViewChange` messages will no longer contain logs, but will contain all the other information from the original protocol. The replica receiving the `DoViewChange` messages, hereafter referred to as the **View Change Coordinator**, or simply **Coordinator**, will choose the most up to date replica and send it a `BecomePrimary` message. When the proposed primary receives the `BecomePrimary` message it will send slightly modified `StartView` messages to the rest of the replicas which will then update their logs and become backups in the new view.

### Protocol

1. A replica notices the need for a view change due to a timeout or because it received a `StartViewChange` or `DoViewChange` message with a view greater than its own. It broadcasts the following message (which is unchanged from the original protocol) to all replicas:

```rust
struct StartViewChange {
    epoch: u64,
    view: u64,
    from: Pid
}
```

2. When a replica receives a quorum (including itself) of `StartViewChange` messages from distinct replicas, it sends a `DoViewChange` message to the coordinator of the new view, using the existing round robin strategy to select the coordinator. Note that this message only sends the last log entry as described in *Section 5.3* of the VRR paper, which discusses optimizing view changes. If this entry is enough to bring the Coordinator up to date, the coordinator becomes the primary and sends a `StartView` message to each replica as described in **step 4**. Otherwise, the coordinator proceeds to **step 3**.

```rust
struct DoViewChange {
    epoch: u64,
    view: u64,
    op: u64,
    from: Pid,
    last_normal_view: u64,
    commit_num: u64,
    last_log_entry: VrMsg
}
```

3. When a coordinator determines it is out of date and cannot catch up with the latest log entry, it determines which of the replicas that sent it the `DoViewChange` message is most up to date and sends it a `BecomePrimary` message. The coordinator decides which replica is the most up to date in the same way it would decide which log is most up to date in the original protocol: The replica with the largest `last_normal_view` is selected. If several replicas share the largest `last_normal_view`, the one with the largest `op` is chosen.

The coordinator at this point has received messages from at least `F` replicas. For these replicas, and itself, it knows precisely which log entries they are missing. If the `last_normal_view` of a replica is the largest received in the quorum, then the missing log entries are the entries after the replica's current op number up to the the highest op number of the `last_normal_view`, which is the last entry in what will be the new primary. If however, the `last_normal_view` of a replica is smaller than that of the new primary, then all entries after that replica's `commit_num` are invalid, as they could be from a view where they were never replicated to a quorum of nodes, and hence reordered during a view change. This strategy of determining what log entries are missing is exactly that used by state transfer as described in section 5.2 of the VRR paper.

The coordinator bundles this information for replicas that it knows about and includes it in the `BecomePrimary` message so that the new primary can refresh the state of the new backups by sending the minimal data required.

There is still the problem that replicas which the coordinator does not have information about cannot be sent the minimal amount of log entries required to bring them up to state. In the original protocol, the entire log is shipped from the primary. As that is precisely what these optimizations are trying to prevent, it is unsuitable for our purposes. Another solution will be discussed in the following steps.

```rust

struct MissingMessages {
    pid: Pid,
    start_op: u64,
}

struct BecomePrimary {
    epoch: u64,
    view: u64,
    from: Pid,
    commit_num: u64,
    missing: Vec<MissingMessages>
}
```

4. At this point, either the coordinator is up to date, or the proposed primary has received a `BecomePrimary` message. The proposed primary will only accept a `BecomePrimary` message if its epoch and view numbers match that of the sender and the sender is the coordinator as determined by the round robin algorithm.

As described in the paper, the new primary takes the largest `commit_num` from the `DoViewChange` messages, which also gets passed along in the `BecomePrimary` message, and sets that as its `commit_num`. It then changes its status to `normal` (primary state in our code) and sends a `StartView` message to each replica. It then commits any uncommitted operations up to the new `commit_num` and starts accepting client requests.

Note that, different from the original protocol, the entire log is not shipped to each replica. Instead only the missing entries are shipped to replicas that the primary knows about. For the replicas that the primary does not know about, it utilizes the same strategy as in the `DoViewChange` messages. It sends the `StartView` message with only the last log entry, and an extra field containing the `last_normal_view` of the primary. This allows the replicas that only need the last entry to catch up and become backups. If not, they can move directly to state transfer.

```rust
struct StartView {
    epoch: u64,
    view: u64,
    commit_num: u64,
    start_op: u64,
    last_normal_view: u64, // only used if not a known replica
    log_tail: Option<Vec<VrMsg>>, // Set only if a known replica
    last_log_entry: Option<VrMsg>
}
```

5. When a replica receives a `StartView` message it first checks to see whether it is receiving a log tail. If so, it truncates its log at `start_op` and appends the `log_tail`. The replica then proceeds as in the original protocol: it sets its op number to the last entry in the log, sets its view number to the one in the message, and sets its status to normal. If their are uncommitted entries in the log, the replica sends `PrepareOk` messages to the primary, executes the committed operations and advances the `commit_num`.

If the replica is receiving a `last_log_entry`, it means that the primary does not know what log entries it is missing. If it is only missing a single log entry, because the `start_op` in the `StartView` message is equal to its op number and its `last_normal_view` is equal to the one in the `StartView` message, then it appends this entry and proceeds as above. Alternatively, if this replica has the same `last_normal_view` as that in the message, but a longer log, it truncates up to `start_op` + 1. This scenario can occur if this replica was the primary or a backup in the `last_normal_view` and prepared messages that did not reach a quorum of replicas before the view change. Subsequently the coordinator didn't get a `DoViewChange` message from this replica and therefore did not consider it to become the primary for the new view.

Lastly, if the replica realizes it is out of date, then it sets its view number to the one in the `StartView` message, and transitions to state transfer. Note that if the replica's `last_normal_view` is the same as that of the `StartView` message, it doesn't have to truncate the log all the way back to its `commit_num`. It can just retrieve all entries after the last entry in its log. This is safe, because log entries in the same view are never re-ordered, and the new primary has either seen more entries from the `last_normal_view` or only has entries from the new view. If the replica's `last_normal_view` is different from that of the `StartView` message it truncates the log to its last committed operation and performs state transfer from there.

### Safety

The updated protocol as written here doesn't change the safety property as described in section 8.1 of the VRR paper:
> The correctness condition for view changes is that every committed
 operation survives into all subsequent views in the same position in the
 serial order.

In the first major change, the coordinator determines the latest state of the log in the same fashion as the primary determines the latest state of the log in the original protocol. The protocol proceeds normally (but without the logs in the `DoViewChange` messages) and then the `Coordinator` deligates its primary role to the replica with the most up to date log. This is safe because only the coordinator can deligate the primary, it only does so after going through the normal view change protocol where it learns from a quorum of replicas, and the delegated primary only accepts the `BecomePrimary` message if it is in the same view as the coordinator meaning that it has participated in this specific view change.

The other major change is that the entire log is not sent in `StartView` messages. For replicas where the missing entries are known, those exact entries are sent in the `log_tail` in the `StartView` message from the new primary, essentially preventing the sending of duplicate entries as in the original protocol that sends the whole log. For replicas where the missing entries are not known, they only maximally truncate their logs back to the last commit point, and then enter state transfer with the new primary. This has the same effect as if they were partitioned off from the new view for a while and the new primary already started processing operations. When the partition healed, the replica would notice the later view via a `Prepare` or `Commit` message and enter state transfer in an identical manner.

### Liveness

Liveness analysis is the same as in section 8.1 of the VRR paper. The one addition is that the proposed primary may not receive the `BecomePrimary` message. In this case one of the replicas will timeout and a new view change will occur.

# Advantages
This scheme has the advantage of always shipping the minimal state between replicas during view changes.

# Drawbacks
The main drawback is that some extra messages may be introduced, namely: `BecomePrimary` and state transfers.

# Alternatives
The VRR paper makes a suggestion in section 5.3 to prevent sending the entire log in `DoViewChange` messages and only send the last entry or two as we do in this RFC. It then suggests that the primary participate in state transfer to get up to date. This has a problem though, in that replicas are only supposed to respond to `GetState` messages when in `normal` mode, and not in `view change` state (per section 5.2 of the VRR paper). Therefore in order to implement this operation, we must either add a new message `ViewChangeGetState` or allow replicas in view change state to respond to `GetState` messages, but only when they come from the proposed primary in the same view.

One more issue to consider with the suggestion from the paper is that while transferring state, we don't want the other replicas to time out and start a new view change. This could be a problem if the state to transfer is large and it takes longer than the view_change timeout to transfer all the data. A fix to this problem is to have the proposed primary periodically send `ContinueWaitForStartView` messages while it is updating it's state, but before it becomes the primary. This introduces a new message to the protocol, but should be safe as it is just a keep alive. This particular solution also has a problem, in that now the system is delayed for a very long view change. Maybe we should allow another view change to take place while the old proposed primary is still catching up, but this raises another question. What happens when we introduce streaming `NewState` messages for large state transfers. Do these operate in line with the consensus protocol? Once the stream has begun, does that replica go back to participating normally? Can it elect a new leader while an on-going `NewState` transfer is occurring?

I favor the solution in this RFC to the solution in the paper, because it doesn't require extra worrying about timeouts during view change and doesn't require state transfer from a replica in view change to the primary. All `GetState` requests are answered from replicas in normal mode. Furthermore, the solution in the paper doesn't even mention preventing shipping the entire log in `StartView` messages. In the original protocol this log is shipped to all other nodes, whereas in this RFC, the minimal state is transferred to `F` nodes, and only remaining nodes *might* participate in state transfer. We could probably use the solution for that problem from this RFC though, even if we kept the rest of the changes in the paper.

A second alternative not mentioned in the paper is to have the primary send the `epoch`, `view`, `op`, `commit_num`, and `last_normal_view` that it knows of for all replicas in every single prepare message, or every nth message. Then view change could proceed as normal, but each replica would only send the maximum possible missing messages in `DoViewChange` to the proposed primary from what they learned earlier. The proposed primary could then send only the maximum amount of possible missing messages to the replicas in the `StartView` message. This has the overhead of adding 40 bytes to every prepare during normal processing, which could add up to a lot of extra network usage. It also requires a bit more state to send during view change, since all nodes are now sending `log_tail`s to the proposed primary in `DoViewChange` messages. It also has the pathological case where a long partitioned node requires significant state transfer when becoming the primary. It does however have the benefit of mainly sticking with the protocol in the paper and avoiding unknowns with the other changes. It is also quite simple.

# Unresolved Questions

* Are the proposed changes actually safe?
* What happens when we start garbage collecting the log?
  * I think that this is only a problem for state transfer, and that would act in the same manner as described in the paper.
  * For alternative 2, we could actually prevent garbage collecting the log until all the nodes have caught up to a specific **GC Point**. However, we may have memory pressure that prevents us from waiting forever on long partitioned nodes.
* Should large state transfers happen outside the consensus fsm, once they are started?
