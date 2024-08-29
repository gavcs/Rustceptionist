//! Rustceptionist is the web server that handles 3-4 html documents, implementing both the GET and POST (not currently functioning) HTTP methods.
//! The root document is '/html/troll.html'. This document asks the user who requests the page for a password. If the correct password ("chom") is entered,
//! the user will be redirected to another page. The entered password would be sent through a POST HTTP packet. The idea would be to change something within a
//! secret HTML document that's cleverly titled '/html/secret.html' that can be accessed by simply requesting 127.0.0.1:7399/secret. When the correct password
//! is entered, they wold be redirected to '/html/updater.html'.

use std::{
    fs, io, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

/// Method currently has 2 possible values, GET and POST. Method represents the different HTML methods that are represented in this web server
#[derive(PartialEq)]
pub enum Method {
    GET,
    POST,
}


/// when the thread is spawned, it uses the handle function with the TcpStream that is used as a parameter. It will parse the header, determining the method of
/// the HTTP packet and responding accordingly.
fn handle(mut stream: TcpStream) {
    // create a BufReader from the stream
    let buf = BufReader::new(&mut stream);

    // the lines() creates a Lines iterator, which is then mapped. This returns an iterator that can be collected
    let mut lines = buf.lines().map(|result| result.unwrap());

    let mut passlen = 0;
    // collect the header into a vector
    let req: Vec<_> = lines.by_ref()
        
        // take_while will check each line and grab each one until it finds an empty line
        .take_while(|line| {
            if line.contains("Content-Length: ") {
                dbg!(line);
                passlen = line[17..].parse().unwrap();
            };
            !line.is_empty()
        })
        // collect puts all of the lines within a vector
        .collect();

    // get the first line of the vector. This will contain the HTTP method
    let m = match req.get(0) {
        
        // use the first word on the first line to determine the HTTP method and return the correct enum
        Some(val) => {
            if val.len() >= 4 && &val.as_str()[0..4] == "POST" {
                Method::POST
            } else if val.len() >= 3 && &val.as_str()[0..3] == "GET" {
                    Method::GET
            } else {
                eprintln!("Couldn't find method, first line = {val}");
                return;
            }
        }
        None =>  {
            eprintln!("Couldn't find first line");
            return;
        }
    };
    
    // this creates a tuple with the status and filename as the parameters. The cases are determined by the Method enum
    let (status, fname) = match m {

        Method::GET => {
            // get the first line of the header and determine if the GET request is requesting a valid page
            let header = req.get(0);
            match header {

                // if the file requested is valid, send the correct information, otherwise send the error page with the 404 NOT_FOUND code
                Some(val) => {
                    match val.as_str() {
                        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/troll.html"),
                        "GET /secret HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/html/secret.html"),
                        _ => {
                            ("HTTP/1.1 404 NOT_FOUND", "./src/html/404.html")
                        }
                    }
                }
                None => {
                    eprintln!("First line doesn't exist");
                    return;
                }
            }
        }
        // TODO: implement password and have POST work properly
        Method::POST => {
            let pword = lines.next().unwrap();
            println!("word: {pword}");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(3));
            println!("ahhhh");
            return;
        }
    };

    // put the entire contents of the file requested into a String (a Result<String, Error>)
    let content = fs::read_to_string(fname);
    let content = match content {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };
    // get the length of the content within the file
    let len = content.len();

    // format the response to send back to the user
    let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

    // write the return packet to the stream to be given to the user
    stream.write_all(response.as_bytes()).unwrap_or_else(|err| {
        eprintln!("{err}");
    });

    // im so eepy
}


fn main() {

    // establish a TcpListener that waits for requests
    //  Loopback address is the ip address and the port number is just "SEXY" using a phone numpad
    let listen = TcpListener::bind("127.0.0.1:7399").unwrap();

    // listen.incoming grabs each of the incoming requests, so this grabs each of the incoming requests
    for stream in listen.incoming() {

        // this unwraps the stream, ignoring errors and grabbing the TcpStream if it's not an error
        let unwrapped_stream = match stream {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("{e}");
                continue;
            },
        };

        // the TcpStream is handled within a thread (TODO: use a ThreadPool)
        thread::spawn(|| {
            handle(unwrapped_stream);
        });
    }
}