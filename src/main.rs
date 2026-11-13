use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Bind to localhost + OS assigned port
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    // Print the socket address (which also exposes the port info)
    dbg!(socket.local_addr().unwrap());

    for stream in socket.incoming() {
        let stream = stream.unwrap();
        // dbg!(&stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
