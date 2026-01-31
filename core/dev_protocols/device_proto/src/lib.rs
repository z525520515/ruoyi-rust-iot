#![allow(non_snake_case)]

pub mod constants;
pub mod frame;
pub mod dev_frame;
pub mod types;
pub mod utils;

pub use constants::{
    FRAME_END, FRAME_START,
    command::{DOWNLINK_DATA, UPLOAD_DATA, UPLOAD_DATA_DEVICE},
};

pub use frame::error::{ParseError, ParseResult};
pub use dev_frame::DevFrame;

// TODO: 重构代码
pub trait Codec<const N: usize> {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Self>
    where
        Self: Sized;

    fn to_bytes_const(&self) -> [u8; N] {
        [0; N]
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_const().to_vec()
    }
}
