use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};


const SERVER_ADDRESS: &str = "localhost:8000";

#[tokio::main]
async fn main(){
   println!("connecting to {}", SERVER_ADDRESS);

   //connecting
   if let Ok(mut stream ) = TcpStream::connect(SERVER_ADDRESS).await{

      println!("connection successful to server {}:{}", 
         stream.local_addr().unwrap().ip(),
         stream.local_addr().unwrap().port()
      );

      //writing to the socket
      let message: &str = "hello from the client hehe";  
      let _ = stream.write_all(message.as_bytes()).await;
      println!("the message sent was: {}", message);

      //read the result
      let mut buffer: [u8; 1024] = [0;1024];
      let length = stream.read(&mut buffer).await.unwrap();
      let return_message = String::from_utf8_lossy(&buffer);
      print!("the received message is: {}", return_message);

   } else{
      println!("failed to pass connect to server. error: {}", SERVER_ADDRESS);
   }

}
