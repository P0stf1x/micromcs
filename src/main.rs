use std::net::TcpListener;

use tracing::trace;

mod game_packet;
mod types;
mod network_handler;
mod network_packet;
mod packed_data;

use crate::network_handler::ConnectionManager;

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let mut connection_manager = ConnectionManager::new();
    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    loop { // SERVER LOOP

        // ACCEPT NEW CONNECTIONS
        'new_connections: loop {
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => {
                        connection_manager.handle_connection(s);
                        trace!("we handled new connection");
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // we never exit listener.incoming() for loop, so breaking out of it
                        break 'new_connections;
                    }
                    Err(e) => panic!("encountered IO error: {e}"),
                }
            }
        };

        // PROCESS ALL CONNECTIONS

        // process clients
        for conn in connection_manager.conns.iter_mut() {
            conn.process();
        }

        // remove closed connections
        let mut index = 0;
        while index < connection_manager.conns.len() {
            if connection_manager.conns[index].is_closed() {
                connection_manager.conns.remove(index);
            } else {
                index += 1;
            }
        }
    }
}
