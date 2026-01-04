use std::net::TcpListener;
use std::io::{Read, Write};

fn main(){
    let _listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        println!("Server running");

        //loop to accept connection
        for stream in _listener.incoming(){
            match stream{
                Ok(mut stream) => {
                    printl!("New client connected");

                    let mut buffer = [0;1024];

                    //read data from client
                    let read_bytes = stream.read(&mut buffer).except("failed to read data.");
                    println!("Received: {}")
                }
                Err(e){
                    println!("error occured: {}",e);
                }
            }
        }
}