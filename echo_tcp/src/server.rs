//client reads a line of text from its standard input and writes it to the server
//server reads the line of text and echoes it back to the client
//client reads the echoed line and prints it in its standard output
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    println!("incoming messages");
    let mut buffer = [0; 512];
    loop {
        let byte_read = stream.read(&mut buffer).expect("Could not read the buffer");
        if byte_read == 0 {
            return Ok(());
        }
        stream.write(&buffer[..byte_read])?;
    }
}

fn main() {
    println!("Hello, world!");
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to the address");
    for stream in tcp_listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("error found: {}", e)
            }
            Ok(answer) => {
                thread::spawn(move || {
                    handle_client(answer).unwrap_or_else(|error| eprintln!("{:?}", error))
                });
            }
        }
    }
}
