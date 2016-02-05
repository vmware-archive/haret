# V2R2

V2R2 is software for providing a distributed service built on proven protocols that exposes strongly consistent coordination primitives via an easy to use API, in order to satisfy the essential dependencies of many distributed applications.

Run the tests with `cargo test`

# Setting up a test cluster

 * Install rust. Some features that v2r2 requires are not yet part of stable or beta builds. The nightly channel of 1.7.0 from 2015-12-13 works. It can be installed by using:
   * curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --date=2015-12-13
 * Install rlwrap so that you can have readline support on CLIs
 * Install the code by cloning fsm, orset, and v2r2 to the same directory. V2R2 will look for
   the fsm code there. This is required since cargo doesn't handle sourcing from private repos.
   * `git clone git@github.com:andrewjstone/orset`
   * `git clone git@github.com:andrewjstone/fsm`
   * `git clone git@github.com:vmware/v2r2`
 * Launch 3 nodes on localhost
   * `cd v2r2 && make launch`
 * Note that all node configuration is stored in `v2r2/dev/devN`
 * Open a second terminal window (output from the launched nodes shows up in the original)
 * Connect to the admin server of node1
   * `rlwrap cargo run --bin v2r2-admin 127.0.0.1:2001`
 * Check the cluster status to verify only a single node (dev1) exists
   * `v2r2-admin> cluster status`
 * Join the nodes together using their cluster ports to form a cluster
   * `v2r2-admin> cluster join 127.0.0.1:3000` (dev2)
   * `v2r2-admin> cluster join 127.0.0.1:4000` (dev3)
 * Create a tenant. The id will be returned to you on the command line
   * `v2r2-admin> vr create tenant r1::dev1,r2::dev2,r3::dev3`
 * Show the tenant info
   * `v2r2-admin> vr tenants`
 * Get the state of r1::dev1 - It should show the primary toward the top.
   * `v2r2-admin> vr replica <TenantId>::r1::dev1`
 * Noting the tenant id and primary from the last step, open a third terminal window
 * Run the CLI client, giving the primary replica to talk to (in this case r2), and the VR API port
   to listen on (in this case 3002).
   * `rlwrap cargo run --bin v2r2-cli-client <UUID> 127.0.0.1:3002`
 * Start issuing commands

```
v2r2> create binary /foo
   ok
   v2r2> list /
   /foo

   v2r2> get /foo

   v2r2> put /foo "hello"
   ok
   v2r2> get /foo
   "hello"
   v2r2> get /foo cas
   CAS: 5
   "hello"
   v2r2> put /foo "hello" 4
   CAS on /foo failed. Expected: 4, Actual: 5
```

 * Note the v2r2 state
 * Kill the master (in this case dev2)
   * `make stop-dev2`
 * Wait for re-election and find the new primary. There is currently no immediate way to get that
   info. You can query replicas via admin nodes or go back to the first window and look at the
   output logging.
 * Connect to the new master (in this case dev3) with the cli client and run some operations, ensuring the state is
   correct
   * `rlwrap cargo run --bin v2r2-cli-client <UUID> 127.0.0.1:4002`

```
v2r2> get /foo cas
CAS: 5
```







# Using the debugger

 * Make a change to a fuzz test so that it fails.
 * Run the test: `cargo test stable` (stable is the prefix of the test name inside `static_membership` fuzz test file)
 * Ensure `rlwrap` is installed so that you can up/down arrow old commands
 * Run the debugger with rlwrap and the proper schedule.txt file:
   `rlwrap target/debug/debugger_cli tests/output/static_membership/current/schedule.txt`
 * Get usage by typing `help` at the prompt (or anything that isn't a command)

Note that output of the latest test run always goes to the `current` directory of the fuzz test. The
name of the failing test history file is always `schedule.txt`.
