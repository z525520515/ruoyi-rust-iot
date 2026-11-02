use bytes::{BytesMut, BufMut};
use crate::error::{Error, Result};

pub trait Encoder {
    type Item;
    
    fn encode(&self, item: Self::Item) -> Result<BytesMut>;
}

pub trait Decoder {
    type Item;
    
    fn decode(&self, buf: &mut BytesMut) -> Result<Option<Self::Item>>;
}

pub struct LengthPrefixedCodec;

impl Encoder for LengthPrefixedCodec {
    type Item = Vec<u8>;
    
    fn encode(&self, item: Self::Item) -> Result<BytesMut> {
        let mut buf = BytesMut::with_capacity(item.len() + 4);
        buf.put_u32(item.len() as u32);
        buf.put_slice(&item);
        Ok(buf)
    }
}

impl Decoder for LengthPrefixedCodec {
    type Item = Vec<u8>;
    
    fn decode(&self, buf: &mut BytesMut) -> Result<Option<Self::Item>> {
        if buf.len() < 4 {
            return Ok(None);
        }
        
        let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        
        if buf.len() < 4 + len {
            return Ok(None);
        }
        
        let data = buf[4..4+len].to_vec();
        let _ = buf.split_to(4 + len);
        
        Ok(Some(data))
    }
}

pub struct StringCodec;

impl Encoder for StringCodec {
    type Item = String;
    
    fn encode(&self, item: Self::Item) -> Result<BytesMut> {
        LengthPrefixedCodec.encode(item.into_bytes())
    }
}

impl Decoder for StringCodec {
    type Item = String;
    
    fn decode(&self, buf: &mut BytesMut) -> Result<Option<Self::Item>> {
        match LengthPrefixedCodec.decode(buf)? {
            Some(data) => {
                let s = String::from_utf8(data).map_err(|e| Error::Codec(format!("Invalid UTF-8: {}", e)))?;
                Ok(Some(s))
            }
            None => Ok(None),
        }
    }
}
