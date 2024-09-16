use std::net::UdpSocket;
use std::io::stdin;
use std::str;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not bind to the IP");
    socket.connect("127.0.0.1:5000").unwrap();
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read from stdin");
        socket.send(input.as_bytes()).expect("Failed to write to the socket");
        let mut buff = [0u8;1500];
        socket.recv_from(&mut buff).expect("Could not read into buffer");
        println!("Received data: {}", str::from_utf8(&mut buff).expect("Could not write buffer as string"));

    }
}