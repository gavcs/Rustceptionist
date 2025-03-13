use std::{
    io::{
        prelude::*,
        BufReader
    },
    net::TcpStream,
    fmt::Display,
    fs
};


#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    UNKNOWN
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m: &str = "";
        if self == &HttpMethod::GET {
            m = "GET";
        } else if self == &HttpMethod::POST {
            m = "POST";
        } else {
            m = "UNKNOWN";
        }

        write!(f, "{}", m)
    }
}

fn get (path: &str) -> String {
    let valid: bool = match path {
        "/" => {
            true
        },
        _ => false
    };

    let ret = if valid {
        let fullpath = match path {
            "/" => "./src/head/html/index.html",
            _ => "html/404.html"
        };
        let content = fs::read_to_string(fullpath);
        let content = match content {
            Ok(val) => val,
            Err(e) => {
                println!("{e} for {fullpath}\nThis is the path: {path}");
                panic!();
            }
        };
        let len = content.len();
        format!("HTTP/1.1 200 OK\r\nContent-Length: {len}\r\n\r\n{content}")
    } else {
        format!("HTTP/1.1 404 NOT_FOUND\r\nContent-Length: 0\r\n\r\n")
    };

    return ret;
}

fn post(path: &str, content: String) -> String {
    return String::from("unimplemented");
}

fn posterr(path: &str) -> String {
    return String::from("unimplemented");
}

fn reterr(path: &str) -> String {
    return String::from("unimplemented");
}

fn get_response (method: HttpMethod, path: &str, content: Option<String>) -> String {
    println!("[Debug] METHOD: {}", method);
    let s = match content {
        Some(s) => s,
        None => return posterr(path)
    };
    println!("[DEBUG] CONTENT: {}", s);
    let ret = match method {
        HttpMethod::GET => get(path),
        HttpMethod::POST => post(path, s),
        _ => {
            reterr(path)
        }
    };
    return ret;
}

pub fn handle_client(mut stream: TcpStream) {
    let buff = BufReader::new(&mut stream);
    let lines = buff.lines().map(|result| result.unwrap());
    let req: String = lines.collect();
    let request: Vec<&str> = req.split(' ').collect();
    let path: &str = request.get(1).unwrap_or_else(||{
        eprintln!("THIS IS ANNOYING!!!\n\tNone");
        &"aaa"
    });
    
    let method = match request.get(0) {
        Some(s) => {
            match s{
                &"GET" => {
                    HttpMethod::GET
                },
                &"POST" => {
                    HttpMethod::POST
                },
                _ => {
                    HttpMethod::UNKNOWN
                }
            }
        },
        None => {
            println!("\t[DEBUG] UNKNOWN Request Received");
            HttpMethod::UNKNOWN
        }
    };

    let mut headcheck = false;
    let mut header = String::new();
    let mut content = String::new();
    for line in request.iter() {
        if line.eq(&String::from("\r\n")) {
            headcheck = true;
            continue;
        } else if headcheck == true {
            content += &line;
        } else {
            header += &line;
        }
    }

    let response = get_response(method, path, match content.is_empty() {
        true => Some(content),
        false => None
    });

    stream.write_all(response.as_bytes()).unwrap_or_else(|err| {
        eprintln!("{err}");
    });
}