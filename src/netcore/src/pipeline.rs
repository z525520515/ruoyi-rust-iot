use std::sync::Arc;
use crate::handler::{ChannelHandlerContext, InboundHandler, OutboundHandler, BoxedAny};
use crate::error::Result;

pub struct Pipeline {
    inbound_handlers: Vec<Arc<dyn InboundHandler + Send + Sync + 'static>>,
    outbound_handlers: Vec<Arc<dyn OutboundHandler + Send + Sync + 'static>>,
    context_inbound: Vec<ChannelHandlerContext>,
    context_outbound: Vec<ChannelHandlerContext>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            inbound_handlers: Vec::new(),
            outbound_handlers: Vec::new(),
            context_inbound: Vec::new(),
            context_outbound: Vec::new(),
        }
    }

    pub fn add_last_inbound<H: InboundHandler + 'static>(&mut self, handler: Arc<H>) {
        self.inbound_handlers.push(handler);
        self.context_inbound.push(ChannelHandlerContext::new());
    }

    pub fn add_last_outbound<H: OutboundHandler + 'static>(&mut self, handler: Arc<H>) {
        self.outbound_handlers.push(handler);
        self.context_outbound.push(ChannelHandlerContext::new());
    }

    pub async fn fire_channel_read(&self, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        let mut current_msg = msg;
        
        for (handler, ctx) in self.inbound_handlers.iter().zip(self.context_inbound.iter()) {
            match handler.channel_read(ctx, current_msg).await? {
                Some(m) => current_msg = m,
                None => return Ok(None),
            }
        }
        
        Ok(Some(current_msg))
    }

    pub async fn fire_channel_active(&self) -> Result<()> {
        for (handler, ctx) in self.inbound_handlers.iter().zip(self.context_inbound.iter()) {
            handler.channel_active(ctx).await?;
        }
        Ok(())
    }

    pub async fn fire_channel_inactive(&self) -> Result<()> {
        for (handler, ctx) in self.outbound_handlers.iter().rev().zip(self.context_outbound.iter().rev()) {
            handler.channel_inactive(ctx).await?;
        }
        Ok(())
    }

    pub async fn write(&self, msg: BoxedAny) -> Result<Option<BoxedAny>> {
        let mut current_msg = msg;
        
        for (handler, ctx) in self.outbound_handlers.iter().rev().zip(self.context_outbound.iter().rev()) {
            match handler.write(ctx, current_msg).await? {
                Some(m) => current_msg = m,
                None => return Ok(None),
            }
        }
        
        Ok(Some(current_msg))
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
