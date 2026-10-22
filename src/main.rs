use std::io::{Read, Write};
use std::println;
use std::net;
use std::net::TcpStream;

fn handle_client(mut stream: TcpStream) {
    let mut buff = [0; 8192];
    let res = stream.read(&mut buff);

    match res {
        Ok(0) => {},
        Ok(size) => {
            println!("Received {size} bytes");
            stream.write(&buff[0..size]).unwrap();
        }
        Err(_) => {}
    }
}

fn main() -> std::io::Result<()> {

    println!("Hello from my_RDS");

    let listener = net::TcpListener::bind("0.0.0.0:4242")?;

    loop {
        for stream in listener.incoming() {
            handle_client(stream?);
        }
    }
    return Ok(());
}
