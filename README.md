# V2R2

V2R2 is software for providing a distributed service built on proven protocols that exposes strongly consistent coordination primitives via an easy to use API, in order to satisfy the essential dependencies of many distributed applications.

Run the tests with `cargo test`

# Using the debugger

 * Make a change to a fuzz test so that it fails.
 * Run the test: `cargo test stable` (stable is the prefix of the test name inside `static_membership` fuzz test file)
 * Ensure `rlwrap` is installed so that you can up/down arrow old commands
 * Run the debugger with rlwrap and the proper schedule.txt file:
   `rlwrap target/debug/debugger_cli tests/output/static_membership/current/schedule.txt`
 * Get usage by typing `help` at the prompt (or anything that isn't a command)

Note that output of the latest test run always goes to the `current` directory of the fuzz test. The
name of the failing test history file is always `schedule.txt`.
