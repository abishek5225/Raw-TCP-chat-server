use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    let listner = TcpListener::bind("127.0.0.1.8080").expect("Could not bind");
    let clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server started on 127.0.0.1.8080",)
    for stream in listner.incoming(){
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);

                clients.lock().unwrap().push(stream.try_clone().except("Failed to clone client"));

                thread::spawn(move || {
                    handle_client(stream, clients);
                })
            }
            Err(e) => println!("Error: {}", e),
            
        }
    }
}
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>){
    let mut buffer = [0; 512];
     
    while match stream.read(&mut buffer){

    Ok(size){
        if size == 0 {
            return;
        }
    let message = &buffer[0..size];
    let mut clients_guard = clients.lock().unwrap();

            for mut client in clients_guard.iter() {
                client.write_all(message).expect("Failed to broadcast");
            }
            true
        }
        Err(_) => {
            println!("An error occurred with a client");
            false
    }
    }{}
}