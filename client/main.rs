use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8080").except("Couldn't connect to server");

    println!("connected to the fucking server");

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop{
            
        }
    })
}