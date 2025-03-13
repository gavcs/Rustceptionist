use std::{net::TcpListener, thread};

use super::http_helper::{self};

pub fn listen () {
    let listen = TcpListener::bind("localhost:8080").unwrap();
    
    for stream in listen.incoming() {
        let unwrapped_stream = match stream {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("Error {e}");
                continue;
            }
        };
        thread::spawn(|| {
            http_helper::handle_client(unwrapped_stream)
        });
    }
}