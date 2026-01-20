use std::any::Any;
use async_trait::async_trait;
use crate::error::Result;

pub type BoxedAny = Box<dyn Any + Send + Sync>;

#[derive(Debug, Clone)]
pub struct ChannelHandlerContext;

impl ChannelHandlerContext {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
pub trait ChannelHandler: Send + Sync {
    async fn channel_active(&self, _ctx: &ChannelHandlerContext) -> Result<()> {
        Ok(())
    }

    async fn channel_inactive(&self, _ctx: &ChannelHandlerContext) -> Result<()> {
        Ok(())
    }

    async fn exception_caught(&self, _ctx: &ChannelHandlerContext, error: crate::Error) -> Result<()> {
        eprintln!("Exception caught: {:?}", error);
        Ok(())
    }
}

#[async_trait]
pub trait InboundHandler: ChannelHandler {
    async fn channel_read(&self, _ctx: &ChannelHandlerContext, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        Ok(Some(msg))
    }
}

#[async_trait]
pub trait OutboundHandler: ChannelHandler {
    async fn write(&self, _ctx: &ChannelHandlerContext, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        Ok(Some(msg))
    }
}
