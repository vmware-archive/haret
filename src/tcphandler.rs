use mio;
use event_loop::Notification;
use state::State;
use resp::Parse;
use event::Event;
use std::sync::mpsc::Sender;

pub trait TcpHandler {
    type TcpMsg: Parse + Send;

    // Sets the channel the handler sends messages to the event loop on
    fn set_event_loop_tx(&mut self, tx: mio::Sender<Notification>);

    // Returns the channel the event loop sends messages to the handler on
    fn event_loop_sender(&self) -> Sender<Event<Self::TcpMsg>>;

    fn host(state: &State) -> String;
    fn recv_loop(&mut self);
}
