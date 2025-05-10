use std::str::FromStr;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

const PATTU_SERVER_ADDRESS: &str = "127.0.0.1:8000";
const KAREN_SERVER_ADDRESS: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() {

    //starting
    println!("Karen server starting on {}", KAREN_SERVER_ADDRESS);

    //binding to port
    if let Ok(socket) = TcpListener::bind(KAREN_SERVER_ADDRESS).await{
        println!("Karen listening on {}", KAREN_SERVER_ADDRESS); 

        // for stream in socket.incoming(){
        //     let mut _stream = stream.unwrap();
        //     println!("Connection Established to: {}:{}", _stream.peer_addr().unwrap().ip(), _stream.peer_addr().unwrap().port());
        //     handle_connection(&mut _stream);
        // }

        loop{
            let mut stream = socket.accept().await.unwrap();
            tokio::spawn(async move{handle_connection(&mut stream.0).await});
            // handle_connection(&mut stream.0).await;
        }
    }else{
        println!("Error listening on {}", KAREN_SERVER_ADDRESS);
    }
}

async fn handle_connection(stream: &mut TcpStream){
    //read the buffer
    let mut buffer: [u8;1024] = [0;1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}", message);

    //call to pattu
    let pattu_message = call_pattu(message.to_owned().to_string()).await;
    let output = format!("pattu says: {}",pattu_message);
    
    //write a message
    let _ = stream.write_all(output.as_bytes()).await;
    println!("reply sent was: {}", output);
}

async fn call_pattu(message: String) -> String{
   println!("connecting to pattu: {}", PATTU_SERVER_ADDRESS);

   //connecting
   if let Ok(mut stream ) = TcpStream::connect(PATTU_SERVER_ADDRESS).await{
      println!("connection successful to pattu{}:{}", 
         stream.local_addr().unwrap().ip(),
         stream.local_addr().unwrap().port()
   );

   //writing to the socket
   let _ = stream.write(message.as_bytes()).await;
   let _ = stream.flush();
   println!("the message sent to pattu: {}", message);

   //read the result
   let mut buffer: [u8; 1024] = [0;1024];
   let _length = stream.read(&mut buffer).await.unwrap();
   let return_message = String::from_utf8_lossy(&buffer);
   print!("the received message from pattu is: {}", return_message);

   //return a message
   return message.to_owned().to_string();

   } else{
      println!("failed to pass connect to server. error: {}", PATTU_SERVER_ADDRESS);
      return String::from_str("failed to connect to pattu").unwrap();
   }

}
