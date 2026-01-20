use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use crate::pipeline::Pipeline;
use crate::error::{self, Result};

pub struct Channel {
    stream: Option<TcpStream>,
    pipeline: Pipeline,
    is_active: bool,
}

impl Channel {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Some(stream),
            pipeline: Pipeline::new(),
            is_active: true,
        }
    }

    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if let Some(stream) = &mut self.stream {
            loop {
                match stream.try_read(buf) {
                    Ok(n) if n > 0 => return Ok(n),
                    Ok(0) => return Ok(0),
                    Ok(_) => {
                        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                        continue;
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        stream.readable().await.map_err(error::Error::from)?;
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        } else {
            Err(Error::other("Stream is None").into())
        }
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            tokio::io::AsyncWriteExt::write_all(stream, buf).await.map_err(Error::from)?;
            Ok(())
        } else {
            Err(Error::other("Stream is None").into())
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn close(&mut self) {
        self.is_active = false;
        self.stream = None;
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            tokio::io::AsyncWriteExt::shutdown(&mut stream).await.map_err(Error::from)?;
        }
        self.is_active = false;
        Ok(())
    }
}

pub struct ChannelBuilder {
    channel: Channel,
}

impl ChannelBuilder {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            channel: Channel::new(stream),
        }
    }

    pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
        self.channel.pipeline = pipeline;
        self
    }

    pub fn build(self) -> Channel {
        self.channel
    }
}
