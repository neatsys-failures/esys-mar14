use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:50000").await.unwrap();
    let mut buf = [0; 2048];
    loop {
        let (_, remote) = socket.recv_from(&mut buf).await.unwrap();
        socket.send_to(&[], remote).await.unwrap();
    }
}
