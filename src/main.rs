use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Bind to localhost + OS assigned port
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    // Print the socket address (which also exposes the port info)
    dbg!(socket.local_addr().unwrap());

    for stream in socket.incoming() {
        let stream = stream.unwrap();
        dbg!(&stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}")
}
