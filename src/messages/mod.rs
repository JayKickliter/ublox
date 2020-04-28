//! u-blox message types.
pub mod ack;
pub mod cfg;
pub mod nav;
pub mod primitive;
use crate::framing::Frame;
use ack::AckNak;
use cfg::Cfg;
use nav::Nav;

/// Top-level enum for valid u-blox messages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Msg {
    /// Ack/Nak
    AckNak(AckNak),
    /// Configuration message.
    Cfg(Cfg),
    /// Navigation message.
    Nav(Nav),
}

impl Msg {
    /// Parses a u-blox message from a [`Frame`].
    pub fn from_frame(frame: &Frame) -> Result<Self, ()> {
        match frame.class {
            cfg::Cfg::CLASS => Ok(Msg::Cfg(Cfg::from_frame(frame)?)),
            nav::Nav::CLASS => Ok(Msg::Nav(Nav::from_frame(frame)?)),
            ack::AckNak::CLASS => Ok(Msg::AckNak(AckNak::from_frame(frame)?)),
            _ => Err(()),
        }
    }
}

/// Represents any u-blox protocol message.
pub trait Message: Sized {
    /// Message Class.
    const CLASS: u8;
    /// Message ID.
    const ID: u8;
    /// Message length.
    const LEN: usize;

    /// Serialize message bytes to a buffer.
    fn serialize<B: bytes::BufMut>(&self, dst: &mut B) -> Result<(), ()>;

    /// Deserialize a message from buffer of a bytes.
    fn deserialize<B: bytes::Buf>(src: &mut B) -> Result<Self, ()>;
}
