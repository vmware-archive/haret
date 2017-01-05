# V2R2

V2R2 provides a distributed service built on proven protocols that exposes strongly consistent coordination primitives via an easy to use API, in order to satisfy the essential dependencies of many distributed applications. You can read more about why we decided to build V2R2 [here](https://github.com/vmware/v2r2/blob/master/docs/why.md).

This README contains quickstart instructions for both developers and users. More information
about using V2R2 can be found in the *rough and incomplete* [User
Guide](https://github.com/vmware/v2r2/blob/master/docs/v2r2-user-guide.md).

# Release Quickstart
Start Here if you are an enduser of V2R2 and not interested in building the code.

## Running a replication group on a single node
Release binaries for `Mac OSX` and `Linux` can be downloaded
[here](https://github.com/vmware/v2r2/releases).

Each release directory contains a `config.json` file for running a single node of v2r2. From the release directory
containing the `config.json` and binaries, run `v2r2` in a terminal with the following command to
start a node:

```
RUST_LOG=v2r2=info=rabble=info ./v2r2
```

While a V2R2 replication group (namespace) is intended to run in a cluster across multiple physical
machines for fault tolerance, it is capable of running in a single process for testing purposes.
Each replica in the group is represented by a lightweight actor that can receive and respond to
messages, so we can just designate a group of actors on the same node to be the replication group.

Open another terminal that we can use to communicate with this node.

Connect to the admin server of node1

```
./v2r2-admin 127.0.0.1:5001
```

Check the cluster status to verify only a single node exists

```
v2r2-admin> cluster status
```

Create a namespaces on node1

```
v2r2-admin> vr create namespace test-ns r1::node1@127.0.0.1:5000,r2::node1@127.0.0.1:5000,r3::node1@127.0.0.1:5000

```

List namespaces

```
v2r2-admin> vr namespaces
```

Get the state of r1::dev1 - It will show the primary of the `test-ns` namespace toward the top.

```
v2r2-admin> vr replica test-ns::r1::node1@127.0.0.1:2000`
```

Show Configuration

```
v2r2-admin> config
```

Now exit the admin CLI and connect to the API port on `node1` using the CLI client

```
./v2r2-cli-client 127.0.0.1:5002
```

List namespaces

```
v2r2> list-namespaces
```

Enter the `test-ns` namespace so you can issue operations against the namespace's tree. This entails
learning which replica is the primary for that namespace so it can be sent messages.
```
v2r2> enter test-ns
```

Now start issuing operations against the namespace. They can be discovered by typing `help` at
the prompt. More examples are shown [below](#issuing-commands-against-a-namespace).

If you'd like to build a multi-node cluster all on localhost, it's a simple matter of copying the
release directory `n` times, where `n` is the size of the cluster, and then editing each config file
to listen on different ports (This is exactly what `make devrel` does if you have the build
dependencies). Each v2r2 in each directory can then be started. From there the [instructions
below](#joining-3-nodes-to-create-a-cluster) detail joining nodes together to form a cluster.

# Developer Quickstart
Start here if you are looking to hack on V2R2.

## Setting up a development cluster
 * Install [Rust](https://doc.rust-lang.org/book/). Some features that v2r2 requires are not yet part of stable or beta builds. The nightly channel of 1.16.0 from January 8, 2017 works. It can be installed by using:
   * curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --date=2017-01-08
 * Install [rlwrap](https://linux.die.net/man/1/rlwrap) so that you can have readline support on CLIs
 * Build a 3 node development cluster and launch 3 nodes on localhost
   * `cd v2r2 && make launch`
 * Note that all node configuration is stored in `v2r2/dev/devN`

### Joining 3 nodes to create a cluster
 * Open a new terminal window (output from the launched nodes shows up in the original)
 * Connect to the admin server of node1
   * `rlwrap cargo run --bin v2r2-admin 127.0.0.1:2001`
 * Check the cluster status to verify only a single node (dev1) exists
   * `v2r2-admin> cluster status`
 * Join the nodes together using their cluster ports to form a cluster
   * `v2r2-admin> cluster join dev2@127.0.0.1:3000`
   * `v2r2-admin> cluster join dev3@127.0.0.1:4000`

### Creating and operating on a namespace
 * Create a namespace
   * `v2r2-admin> vr create namespace test-ns r1::dev1@127.0.0.1:2000,r2::dev2@127.0.0.1:3000,r3::dev3@127.0.0.1:4000`
 * List namespaces
   * `v2r2-admin> vr namespaces`
 * Get the state of r1::dev1 - It should show the primary toward the top.
   * `v2r2-admin> vr replica test-ns::r1::dev1@127.0.0.1:2000`
 * Show Configuration
   * `config`
 * Exit the Admin client
 * Run the CLI client, giving the API address of any node. We chose to connect to dev1 here.
   * `rlwrap target/debug/v2r2-cli-client 127.0.0.1:2002`
 * List namespaces
   * `list-namespaces`
 * Enter the `test-ns` namespace so you can issue operations against the namespace's tree
   * `enter test-ns`

### Issuing commands against a namespace

```
v2r2> create set /some/other/node
Ok
Epoch = 1, View = 6, Client Request Num = 3
v2r2> set insert /some/other/node hi
true
Version = 1 Epoch = 1, View = 6, Client Request Num = 4
v2r2> set contains /some/other/node hi
true
Version = 1 Epoch = 1, View = 6, Client Request Num = 5
v2r2> set contains /some/other/node h
false
Version = 1 Epoch = 1, View = 6, Client Request Num = 6
v2r2> create set /some/set
Ok
Epoch = 1, View = 6, Client Request Num = 7
v2r2> set insert /some/set blah
true
Version = 1 Epoch = 1, View = 6, Client Request Num = 8
v2r2> set intersection /some/set /some/other/node
Epoch = 1, View = 6, Client Request Num = 9
v2r2> set union /some/set /some/other/node
hi
blah
Epoch = 1, View = 6, Client Request Num = 10
v2r2>
```

### Test failover
 * Note the v2r2 state
   * `v2r2-admin> vr replica test-ns::r1::dev1`
 * Kill the primary (in this case dev2)
   * `make stop-dev2`
 * Wait a few seconds for re-election to occur and then re-enter the namespace from the client cli.
   to try to discover the new primary.
   * `enter test-ns`
 * Issue new commands that show the state is preserved after crashing the master

```
v2r2> set union /some/set /some/other/node
hi
blah
Epoch = 1, View = 7, Client Request Num = 11
```
