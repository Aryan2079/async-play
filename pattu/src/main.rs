use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::env::args;
use std::{thread, time::Duration};

const PATTU_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    //read arguments 
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default();
    // println!("{}",&delay);

    //starting
    println!("Pattu server starting on {}", PATTU_SERVER_ADDRESS);

    //binding to port
    if let Ok(socket) = TcpListener::bind(PATTU_SERVER_ADDRESS){
        println!("Pattu listening on {}", PATTU_SERVER_ADDRESS); 

        for stream in socket.incoming(){
            let mut _stream = stream.unwrap();
            println!("Connection Established: {}:{}", _stream.peer_addr().unwrap().ip(), _stream.peer_addr().unwrap().port());
            handle_connection(&mut _stream, delay);
        }
    }else{
        println!("Error listening on {}", PATTU_SERVER_ADDRESS);
    }
}

fn handle_connection(stream: &mut TcpStream, delay: u64){
    //read the buffer
    let mut buffer: [u8;1024] = [0;1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}", message);

    thread::sleep(Duration::from_millis(delay));
    
    //write a message
    let reply: &str = "hello from pattu";
    let _ = stream.write_all(reply.as_bytes()).unwrap();
    println!("reply sent was: {}", reply);
}
