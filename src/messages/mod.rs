//! u-blox message types.
pub mod cfg;
pub mod nav;
pub mod primitive;
use crate::framing::Frame;
use cfg::Cfg;
use nav::Nav;

/// Top-level enum for valid u-blox messages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Msg {
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
            _ => Err(()),
        }
    }
}

/// Represents any u-blox protocol message.
pub trait Message {
    /// Message Class.
    const CLASS: u8;
    /// Message ID.
    const ID: u8;
    /// Message length.
    const LEN: usize;

    /// Encode message bytes to a buffer.
    fn to_bytes(&self, _buf: &mut [u8]) -> Result<(), ()> {
        unimplemented!()
    }
}
