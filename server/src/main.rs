use std::net::TcpListener;
use std::io::{Read, Write};

fn main(){
    let _listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        println!("Server running");

}