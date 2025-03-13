use std::io::{stdin,stdout,Write};

pub fn close_server() {
    let mut s = String::new();
    stdin().read_line(&mut s);
    if s == String::from("exit") {
        exit();
    }
}