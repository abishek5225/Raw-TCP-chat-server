use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    loop{
        match stream.read(&mut buffer);
        Ok(0) => {
            println!("CLient Disconnected.");
            break;
        }
        Ok(read_bytes) => {
            let msg = String::from_utf8_lossy(&buffer[..read_bytes]);
            println!("Received: {}", msg);

            stream.write_all(b"message received\n").expect("Failed to write to client");
        }
        Err(e) => {
            println!("Error: {}", e);
            break;
        }
    }
}
fn main(){
    let _listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        println!("Server running");

        //loop to accept connection
        for stream in _listener.incoming(){
            match stream{
                Ok(stream) => {
                    println!("New client connected");

                    let mut buffer = [0;1024];

                    thread::spawn(|| {
                        handle_client(stream);
                    })

             /*       // //read data from client
                    // let read_bytes = stream.read(&mut buffer).expect("failed to read data.");
                    // //convert bytes to string
                    // println!("Received: {}", String::from_utf8_lossy(&buffer[..read_bytes]));

                      // response
                    //b" -> it means byte string coz tcp always send bytes
                    stream.write_all(b"Hello from Rust TCP server!\n")
                    .expect("Failed to write to client");
                    */
                  
                }
                Err(e) => {
                    println!("error occured: {}",e);
                }
            }
        }
}