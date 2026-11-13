use std::net::TcpListener;

fn main() {
    // Bind to localhost + OS assigned port
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    // Print the socket address (which also exposes the port info)
    dbg!(socket.local_addr().unwrap());

    for stream in socket.incoming() {
        let stream = stream.unwrap();
        dbg!(stream);
        println!("Connection established!");
    }
}
