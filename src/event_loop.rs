use mio;
use mio::{Token, Handler, PollOpt, Interest, ReadHint };
use mio::tcp::{TcpListener, TcpStream, TcpSocket, Shutdown};
use tcpserver::TcpServer;

const ACCEPTOR: Token = Token(0);

pub struct EventLoop<T: TcpServer> {
    ctx: Context<T>,
    event_loop: mio::EventLoop<Context<T>>
}

impl<T: TcpServer> EventLoop<T> {
    pub fn new(ipstr: &str, server: T) -> EventLoop<T> {
        let addr = ipstr.parse().unwrap();
        let sock = TcpSocket::v4().unwrap();
        sock.set_reuseaddr(true).unwrap();
        sock.bind(&addr).unwrap();
        let listener = sock.listen(1024).unwrap();
        let mut event_loop = mio::EventLoop::new().unwrap();
        event_loop.register_opt(&listener, ACCEPTOR, Interest::readable(),
                                PollOpt::edge()).unwrap();
        let ctx = Context::new(listener, server);
        EventLoop {
            ctx: ctx,
            event_loop: event_loop
        }
    }

    /// This will block the current thread
    pub fn run(&mut self) {
        self.event_loop.run(&mut self.ctx).unwrap();
    }
}

struct Context<T: TcpServer> {
    last_token: Token,
    listener: TcpListener,
    server: T
}

impl<T: TcpServer> Context<T> {
    fn new(listener: TcpListener, server: T) -> Context<T> {
        Context {
            last_token: Token(1),
            listener: listener,
            server: server
        }
    }

    fn try_accept(&mut self, event_loop: &mut mio::EventLoop<Context<T>>) {
        println!("Waiting to accept a connection");
        match self.listener.accept() {
            Ok(None) => (), // EWOULDBLOCK
            Ok(Some(sock)) => {
                let Token(count) = self.last_token;
                self.last_token = Token(count + 1);
                event_loop.register_opt(&sock, self.last_token,
                                        Interest::readable() | Interest::writable(),
                                        PollOpt::edge()).unwrap();
                self.server.new_sock(self.last_token, sock);
            },
            Err(err) => println!("Error accepting connection: {}", err)
        }
    }
}

impl<T: TcpServer> Handler for Context<T> {
    type Timeout = ();
    type Message = ();

    fn readable(&mut self, event_loop: &mut mio::EventLoop<Context<T>>, token: Token, _: ReadHint)
    {
        match token {
            ACCEPTOR => { self.try_accept(event_loop); }
            _i => { self.server.readable(token); }
        }
    }

    fn writable(&mut self, event_loop: &mut mio::EventLoop<Context<T>>, token: Token) {
        self.server.writable(token);
    }
}

