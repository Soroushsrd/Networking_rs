mod client;

use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:5000").expect("Could not bind to the IP");
    loop {
        let mut buff = [0u8;1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match sock.recv_from(&mut buff) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Received data from: {}", src);
                    sock.send_to(&buff, src).expect("Failed to send data");
                });
            }
            Err(e) => {
                eprintln!("could not receive a datagram: {}", e);
            }
        }
    }
}