use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("localhost:3000") {
    

        stream.write("hello".as_bytes()).unwrap();

        let buffer = [0; 1400];
       
        let raw_bytes = str::from_utf8(&buffer).unwrap();
        let utf8_chars = raw_bytes.trim_end_matches(char::from(0));
        println!(
            "Got response from server:{:?}",
            utf8_chars
        );
    } else {
        println!("error : tcp client could not connect");
    }
}
