use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    let listner = TcpListener::bind("127.0.0.1.8080").expect("Could not bind");
    let clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server started on {}",)
}