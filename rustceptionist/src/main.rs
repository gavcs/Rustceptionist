use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread
};

#[derive(PartialEq)]
enum Method {
    GET,
    POST,
}



fn handle(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);

    // map takes the Result<String, Error> from each line from lines and unwraps each of them
    let mut lines = buf.lines().map(|result| result.unwrap());

    let mut passlen = 0;
    // get an iterator over the lines within the BufReader
    let req: Vec<_> = lines.by_ref()
        
        // take_while will check each line and grab each one until it finds an empty line
        .take_while(|line| {
            if line.contains("Content-Length") {
                passlen = line[15..].parse().unwrap();
            };
            !line.is_empty()
        })
        // collect puts all of the lines within a vector
        .collect();

    println!("made vector");
    
    let m = match req.get(0) {
        Some(val) => {
            if &val.as_str()[0..4] == "POST" {
                Method::POST
            } else if &val.as_str()[0..3] == "GET" {
                Method::GET
            } else {
                println!("AHHHH1");
                println!("{val}");
                return;
            }
        }
        None =>  {
            println!("AHHHH2");
            return;
        }
    };
    
    let (status, fname) = match m {
        Method::GET => {
            for line in &req {
                println!("{line}");
            }
            let header = req.get(0);
            match header {
                Some(val) => {
                    match val.as_str() {
                        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/hello.html"),
                        "GET /secret HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/secret.html"),
                        _ => {
                            println!("{val}");
                            return;
                        }
                    }
                }
                None => {
                    println!("AHHHH3");
                    return;
                }
            }
        }
        Method::POST => {
            for line in req {
                println!("{line}");
            }
            if &(lines.next().unwrap())[8..] == 
            return;
        }
    };

    let content = fs::read_to_string(fname);
    let content = match content {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let len = content.len();

    let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

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