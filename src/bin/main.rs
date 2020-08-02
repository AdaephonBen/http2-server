use std::fs;
use tokio::prelude::*;
use tokio::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};

async fn handle_connection(mut stream: TcpStream, response: &String) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush();
}

#[tokio::main]
async fn main() {
    let contents = fs::read_to_string("/home/vishnu/x.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    let mut listener = TcpListener::bind("127.0.0.1:8078").await.unwrap();
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => {
                let currentResponse = response.clone();
                tokio::spawn(async move {
                    handle_connection(stream, &currentResponse).await;
                });
            }
            Err(e) => eprintln!("Error {:?}", e),
        }
    }
}
