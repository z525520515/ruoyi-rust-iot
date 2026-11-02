use netcore::handler::{InboundHandler, ChannelHandlerContext};
use netcore::error::Result;
use netcore::BoxedAny;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[derive(Clone)]
struct ClientHandler;

#[async_trait::async_trait]
impl InboundHandler for ClientHandler {
    async fn channel_read(&self, _ctx: &ChannelHandlerContext, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        if let Some(s) = msg.downcast_ref::<String>() {
            println!("[Client] Received: {}", s);
        }
        Ok(Some(msg))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Connecting to 127.0.0.1:8080...");
    
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    println!("Connected!");
    
    let message = "Hello, Server!";
    stream.write_all(message.as_bytes()).await?;
    println!("[Client] Sent: {}", message);
    
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("[Client] Received: {}", response);
    
    Ok(())
}
