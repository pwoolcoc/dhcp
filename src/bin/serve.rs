extern crate mio;
extern crate dhcp;

use mio::udp::UdpSocket;

const SERVER: mio::Token = mio::Token(0);

struct DhcpServer {
    server: UdpSocket
}

impl mio::Handler for DhcpServer {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_look: &mut mio::EventLook<DhcpServer>, token: mio::Token, events: mio::EventSet) {
        let mut buf = Vec::with_capacity(1024);
        match token {
            SERVER => {
                self.server.recv_from(&mut buf);
            }
        }
    }
}

fn main() {
    let address = "0.0.0.0:6767".parse().unwrap();
    let server = UdpSocket::bound(&address).unwrap();
}

