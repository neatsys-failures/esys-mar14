use std::{env, net::UdpSocket, time::Instant};

fn main() {
    let server_addr = env::args().nth(1).unwrap();
    let concurrent_count = env::args().nth(2).unwrap().parse::<usize>().unwrap();
    let count = env::args().nth(3).unwrap().parse::<usize>().unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    for _ in 0..concurrent_count {
        socket.send_to(&[], &server_addr).unwrap();
    }

    let start = Instant::now();
    let mut buf = [0; 2048];
    for _ in 0..count {
        socket.recv_from(&mut buf).unwrap();
        socket.send_to(&[], &server_addr).unwrap();
    }

    let duration = Instant::now() - start;
    println!("duration {duration:?}");
    println!(
        "throughput {} packets/second",
        count as f32 / duration.as_secs_f32()
    );
}
