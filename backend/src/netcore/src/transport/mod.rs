use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use crate::channel::Channel;
use crate::pipeline::Pipeline;
use crate::handler::{ChannelHandler, InboundHandler, OutboundHandler, ChannelHandlerContext, BoxedAny};
use crate::error::Result;

#[derive(Clone, Copy)]
pub struct EventLoop;

impl EventLoop {
    pub async fn run(self, listener: TcpListener) -> Result<()> {
        println!("Event loop started, listening for connections...");
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("New connection from: {}", addr);
                    self.handle_connection(stream).await;
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn handle_connection(&self, stream: TcpStream) {
        let mut channel = Channel::new(stream);
        let mut pipeline = Pipeline::new();
        
        let handler = Arc::new(EchoHandler);
        pipeline.add_last_inbound(handler);
        
        channel.pipeline_mut().add_last_inbound(Arc::new(SimpleInboundHandler));
        channel.pipeline_mut().add_last_outbound(Arc::new(SimpleOutboundHandler));

        let mut buf = [0u8; 1024];
        loop {
            match channel.read(&mut buf).await {
                Ok(n) if n == 0 => {
                    println!("Connection closed");
                    break;
                }
                Ok(n) => {
                    let data = &buf[..n];
                    println!("Received: {}", String::from_utf8_lossy(data));
                    
                    if let Err(e) = channel.write_all(data).await {
                        eprintln!("Write error: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
        
        let _ = channel.shutdown().await;
    }
}

struct EchoHandler;

impl ChannelHandler for EchoHandler {}

#[async_trait::async_trait]
impl InboundHandler for EchoHandler {
    async fn channel_read(&self, _ctx: &ChannelHandlerContext, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        println!("EchoHandler received message");
        Ok(Some(msg))
    }
}

struct SimpleInboundHandler;

impl ChannelHandler for SimpleInboundHandler {}

#[async_trait::async_trait]
impl InboundHandler for SimpleInboundHandler {
    async fn channel_read(&self, _ctx: &ChannelHandlerContext, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        Ok(Some(msg))
    }
}

struct SimpleOutboundHandler;

impl ChannelHandler for SimpleOutboundHandler {}

#[async_trait::async_trait]
impl OutboundHandler for SimpleOutboundHandler {
    async fn write(&self, _ctx: &ChannelHandlerContext, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        Ok(Some(msg))
    }
}

pub async fn bind(addr: &str) -> Result<TcpListener> {
    let listener = TcpListener::bind(addr).await.map_err(|e| crate::error::Error::from(e))?;
    Ok(listener)
}

pub struct Bootstrap {
    event_loop: EventLoop,
}

impl Bootstrap {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop {},
        }
    }

    pub async fn bind(&self, addr: &str) -> Result<TcpListener> {
        bind(addr).await
    }

    pub async fn start(&self, listener: TcpListener) -> Result<()> {
        self.event_loop.run(listener).await
    }
}
