/// The replica has left and has told the namespace manager to shut it down.
///
/// It doesn't respond to any messages from here on out
state!(Shutdown {});
