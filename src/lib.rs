use std::net::TcpListener;

use anyhow::Context;

use http_connection::handle_connection;

mod threadpool;

mod http_connection;

pub fn run() -> anyhow::Result<()> {
    // // Bind to loopback interface + OS assigned port
    // let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    // Bind to all interfaces + fixed port
    let socket = TcpListener::bind("0.0.0.0:7879").context("Could not bind to port 7879")?;
    println!(
        "Listening on socket {}",
        socket
            .local_addr()
            .context("Could not retrieve socket address")?
    );

    let pool = threadpool::ThreadPool::new(4);

    // Pass any incoming requests off to handle_connection
    // For sake of testing graceful shutdown, we only handle 4 requests before exiting run,
    // thereby dropping threadpool,
    // thereby calling our nice shutdown code from impl Drop for Threadpool.
    for stream_result in socket.incoming().take(4) {
        // Try to handle the connection
        match stream_result {
            Ok(stream) => {
                pool.execute(Box::new(|| {
                    let connection_result = handle_connection(stream, 10);

                    // We have a connection but couldn't read/write from it,
                    // so just report it and move on
                    if let Err(e) = connection_result {
                        eprintln!("{e:#}");
                    }
                }));

                // let connection_result = handle_connection(stream, 10);
                //
                // // We have a connection but couldn't read/write from it,
                // // so just report it and move on
                // if let Err(e) = connection_result {
                //     eprintln!("{e:#}");
                // }
            }
            // We couldn't make the connection, so report the error and move on.
            e @ Err(_) => eprintln!("Failed to accept connection: {:?}", e),
        };

        // // Try to handle the connection
        // let connection_result = stream_result
        //     .context("Failed to accept connection")
        //     .and_then(|stream| handle_connection(stream, 10));
        //
        // // If we got an error from the connection handler, or could not connect at all,
        // // print it to standard error and continue
        // // We don't want to stop the server over a failed connection, so just report it and move on
        // if let Err(e) = connection_result {
        //     eprintln!("{e:#}")
        // }
    }

    println!("Shutting down.");
    Ok(())
}
