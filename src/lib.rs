use std::fs;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use anyhow::Context;
use std::io::prelude::*;
use std::time::Duration;

pub fn run() -> anyhow::Result<()> {
    // // Bind to loopback interface + OS assigned port
    // let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    // Bind to all interfaces + fixed port
    let socket = TcpListener::bind("0.0.0.0:7879").context("Could not bind to port 7879")?;

    // Print the socket address (which also exposes the port info)
    dbg!(
        socket
            .local_addr()
            .context("Could not retrieve local address of successfully bound socket")?
    );

    // Pass any incoming requests off to handle_connection
    for stream_result in socket.incoming() {
        // Try to handle the connection
        let connection_result = stream_result
            .context("Failed to accept connection")
            .and_then(|stream| handle_connection(stream, 10));

        // If we got an error from the connection handler, or could not connect at all,
        // print it to standard error and continue
        // We don't want to stop the server over a failed connection, so just report it and move on
        if let Err(e) = connection_result {
            eprintln!("{e:#}")
        }
    }

    Ok(())
}

// Tries to read an HTTP request from the stream and send back some HTML.
// Times out after `timeout_secs` seconds
fn handle_connection(mut stream: TcpStream, timeout_secs: u64) -> anyhow::Result<()> {
    // Try to set a timeout on the stream
    // Should always work, but there could be OS errors deep underneath
    stream
        .set_read_timeout(Some(Duration::from_secs(timeout_secs)))
        .context("Could not set timeout on stream")?;

    // Create a reader.
    // In order to pass this to read_request_lines, we'll need to perform a mutable borrow.
    // The reason we don't pass the stream into read_request_lines and create the BufReader there
    // is it may buffer more than just the one request. When that locally created reader drops,
    // we lose the buffered data as well. By doing it this way, we still have access to later reqs.
    let mut stream_reader = BufReader::new(&stream);

    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body
    let http_request: Vec<String> =
        read_request_lines(&mut stream_reader, 1024).context("Could not read stream")?;

    let (status_line, filename) = match http_request.first().map(|s| &s[..]) {
        Some("GET / HTTP/1.1") => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    // let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Try to send the response
    stream
        .write_all(response.as_bytes())
        .context("Could not write to stream")?;

    Ok(())
}

// Tries to grab a complete request from the stream reader.
// If the request is too large, grabs `limit` many bytes.
// Returns an `Err` if any of the request lines could not be read.
// Does not return an `Err` if the limit is reached.
fn read_request_lines(
    reader: &mut BufReader<&TcpStream>,
    limit: u64,
) -> anyhow::Result<Vec<String>> {
    reader
        .take(limit)
        .lines()
        // Take until we get an empty line, i.e., Ok("")
        // Take while is_err_or nonempty, equivalent to !(is_ok_and empty)
        .take_while(|result| !result.as_ref().is_ok_and(|line| line.is_empty()))
        .collect::<Result<Vec<String>, std::io::Error>>()
        .context("Could not read a line of the stream")
}
