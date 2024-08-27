use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread,
};

//fn handle_get()

fn handle(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);
    let req = buf.lines().next();

    let request = match req {
        Some(val) => val,
        None => {
            return;
        }
    };

    let finreq = match request {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let (status, fname) = match &(finreq.as_str())[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/hello.html"),
        "GET /secret HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/secret.html"),
        _ => {
            println!("{finreq}");
            ("HTTP/1.1 404 NOT FOUND", "./src/html/404.html")
        }
    };

    let content = fs::read_to_string("./src/html/hello.html");
    let content = match content {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let len = content.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {len}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap_or_else(|err| {
        eprintln!("{err}");
    });
}

fn main() {
    let listen = TcpListener::bind("127.0.0.1:7878").unwrap();
    

    for stream in listen.incoming() {
        let unwrapped_stream = match stream {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("{e}");
                continue;
            },
        };
        thread::spawn(|| {
            handle(unwrapped_stream);
        });
    }
}