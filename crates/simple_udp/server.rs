use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:50000").unwrap();
    let mut buf = [0; 2048];
    loop {
        let (_, remote) = socket.recv_from(&mut buf).unwrap();
        socket.send_to(&[], remote).unwrap();
    }
}
