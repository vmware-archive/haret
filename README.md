# haret

haret provides a distributed service built on proven protocols that exposes strongly consistent coordination primitives via an easy to use API, in order to satisfy the essential dependencies of many distributed applications. You can read more about why we decided to build haret [here](https://github.com/vmware/haret/blob/master/docs/why.md).

This README contains quickstart instructions for both developers and users. More information
about using haret can be found in the *rough and incomplete* [User
Guide](https://github.com/vmware/haret/blob/master/docs/haret-user-guide.md).

# Release Quickstart
Start Here if you are an enduser of haret and not interested in building the code.

## Running a replication group on a single node
Release binaries for `Mac OSX` and `Linux` can be downloaded
[here](https://github.com/vmware/haret/releases).

Each release directory contains a `config.toml` file for running a single node of haret. From the release directory
containing the `config.toml` and binaries, run `haret` in a terminal with the following command to
start a node:

```
RUST_LOG=haret=info,rabble=info ./haret
```

While a haret replication group (namespace) is intended to run in a cluster across multiple physical
machines for fault tolerance, it is capable of running in a single process for testing purposes.
Each replica in the group is represented by a lightweight actor that can receive and respond to
messages, so we can just designate a group of actors on the same node to be the replication group.

Open another terminal that we can use to communicate with this node.

Connect to the admin server of node1

```
./haret-admin 127.0.0.1:5001
```

Check the cluster status to verify only a single node exists

```
haret-admin> cluster status
```

Create a namespaces on node1

```
haret-admin> vr create namespace test-ns r1::node1@127.0.0.1:5000,r2::node1@127.0.0.1:5000,r3::node1@127.0.0.1:5000

```

List namespaces

```
haret-admin> vr namespaces
```

Get the state of r1::dev1 - It will show the primary of the `test-ns` namespace toward the top.

```
haret-admin> vr replica test-ns::r1::node1@127.0.0.1:2000`
```

Show Configuration

```
haret-admin> config
```

Now exit the admin CLI and connect to the API port on `node1` using the CLI client

```
./haret-cli-client 127.0.0.1:5002
```

List namespaces

```
haret> list-namespaces
```

Enter the `test-ns` namespace so you can issue operations against the namespace's tree. This entails
learning which replica is the primary for that namespace so it can be sent messages.
```
haret> enter test-ns
```

Now start issuing operations against the namespace. They can be discovered by typing `help` at
the prompt. More examples are shown [below](#issuing-commands-against-a-namespace).

If you'd like to build a multi-node cluster all on localhost, it's a simple matter of copying the
release directory `n` times, where `n` is the size of the cluster, and then editing each config file
to listen on different ports (This is exactly what `make devrel` does if you have the build
dependencies). Each haret in each directory can then be started. From there the [instructions
below](#joining-3-nodes-to-create-a-cluster) detail joining nodes together to form a cluster.

# Developer Quickstart
Start here if you are looking to hack on haret.

## Setting up a development cluster
 * Install [Rust](https://doc.rust-lang.org/book/). Haret builds on any stable version after rust 1.15
   * curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=stable
 * Install [rlwrap](https://linux.die.net/man/1/rlwrap) so that you can have readline support on CLIs
 * Build a 3 node development cluster and launch 3 nodes on localhost
   * `cd haret && make launch`
 * Note that all node configuration is stored in `haret/dev/devN`

### Joining 3 nodes to create a cluster
 * Open a new terminal window (output from the launched nodes shows up in the original)
 * Connect to the admin server of node1
   * `rlwrap cargo run --bin haret-admin 127.0.0.1:2001`
 * Check the cluster status to verify only a single node (dev1) exists
   * `haret-admin> cluster status`
 * Join the nodes together using their cluster ports to form a cluster
   * `haret-admin> cluster join dev2@127.0.0.1:3000`
   * `haret-admin> cluster join dev3@127.0.0.1:4000`

### Creating and operating on a namespace
 * Create a namespace
   * `haret-admin> vr create namespace test-ns r1::dev1@127.0.0.1:2000,r2::dev2@127.0.0.1:3000,r3::dev3@127.0.0.1:4000`
 * List namespaces
   * `haret-admin> vr namespaces`
 * Get the state of r1::dev1 - It should show the primary toward the top.
   * `haret-admin> vr replica test-ns::r1::dev1@127.0.0.1:2000`
 * Show Configuration
   * `config`
 * Exit the Admin client
 * Run the CLI client, giving the API address of any node. We chose to connect to dev1 here.
   * `rlwrap target/debug/haret-cli-client 127.0.0.1:2002`
 * List namespaces
   * `list-namespaces`
 * Enter the `test-ns` namespace so you can issue operations against the namespace's tree
   * `enter test-ns`

### Issuing commands against a namespace

```
haret> create set /some/other/node
Ok
Epoch = 1, View = 6, Client Request Num = 3
haret> set insert /some/other/node hi
true
Version = 1 Epoch = 1, View = 6, Client Request Num = 4
haret> set contains /some/other/node hi
true
Version = 1 Epoch = 1, View = 6, Client Request Num = 5
haret> set contains /some/other/node h
false
Version = 1 Epoch = 1, View = 6, Client Request Num = 6
haret> create set /some/set
Ok
Epoch = 1, View = 6, Client Request Num = 7
haret> set insert /some/set blah
true
Version = 1 Epoch = 1, View = 6, Client Request Num = 8
haret> set intersection /some/set /some/other/node
Epoch = 1, View = 6, Client Request Num = 9
haret> set union /some/set /some/other/node
hi
blah
Epoch = 1, View = 6, Client Request Num = 10
haret>
```

### Test failover
 * Note the haret state
   * `haret-admin> vr replica test-ns::r1::dev1`
 * Kill the primary (in this case dev2)
   * `make stop-dev2`
 * Wait a few seconds for re-election to occur and then re-enter the namespace from the client cli.
   to try to discover the new primary.
   * `enter test-ns`
 * Issue new commands that show the state is preserved after crashing the master

```
haret> set union /some/set /some/other/node
hi
blah
Epoch = 1, View = 7, Client Request Num = 11
```
