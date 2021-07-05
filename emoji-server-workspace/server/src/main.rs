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
        let incoming = convert_buffer_to_utf8_string(buffer);

        let outgoing = create_reply_string(incoming);
        stream.write(outgoing.as_bytes()).unwrap();
    }
}

fn convert_buffer_to_utf8_string(buffer: [u8; 1400]) -> String {

    let raw_bytes = str::from_utf8(&buffer).unwrap();
    let utf8_chars = raw_bytes.trim_end_matches(char::from(0));


    return String::from(utf8_chars);
}

fn create_reply_string(incoming: String) -> String {
    let sparkle_heart_emoji_bytes = vec![0xF0, 159, 146, 150];
    let emoji_str = str::from_utf8(&sparkle_heart_emoji_bytes).unwrap();

    let outgoing = incoming.clone() + emoji_str;
    return outgoing;

}

