use std::sync::mpsc::Sender;
use std::fmt;
// Sender<T> is not `Debug`. Implement a wrapper where we can impl Debug so that we don't have to impl
// Debug for the entire state of structs storing Sender<T>. If a struct stores a DebugSender<T> instead of
// Sender<T>, then it can #[derive(Debug)] over the containing struct if the rest of the struct's
// members are `Debug`.
#[derive(Clone)]
pub struct DebugSender<T> {
    sender: Sender<T>
}

impl<T> DebugSender<T> {
    pub fn new(sender: Sender<T>) -> DebugSender<T> {
        DebugSender {
            sender: sender
        }
    }

    pub fn send(&self, msg: T) {
        self.sender.send(msg).unwrap();
    }
}

impl<T> fmt::Debug for DebugSender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sender<T>")
    }
}

// We should never be comparing DebugSenders. However, they may be contained within other structs
// that we want to compare. Just return true so we can utilized #[derive(Eq, PartialEq)] on
// containing structs.
impl<T> PartialEq for DebugSender<T> {
    fn eq(&self, _: &DebugSender<T>) -> bool {
        true
    }
}

impl<T> Eq for DebugSender<T> {}
