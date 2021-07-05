use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let proxy_connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("proxy server running on port 3000");

    read_incoming(proxy_connection_listener);

    
}

fn read_incoming(listener: TcpListener) {
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 1400];
        stream.read(&mut buffer).unwrap();
        convert_buffer(buffer);
        let sparkle_heart = vec![0xF0, 159, 146, 150];
        stream.write(&sparkle_heart).unwrap();
    }
}

fn convert_buffer(buffer: [u8; 1400]) {
    println!(
        "buffer from incoming stream :{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}