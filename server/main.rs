use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    let listner = TcpListener::bind("127.0.0.1.8080").expect("Could not bind");
    let clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server started on {}",)
    for stream in listner.incoming(){
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients)
            }
            Err(e) => println!("Error: {}", e),
            
        }
    }
}