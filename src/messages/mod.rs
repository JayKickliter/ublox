//! u-blox message types.
pub mod nav;
pub mod primitive;
use crate::framing::Frame;
use nav::Nav;

/// Top-level enum for valid u-blox messages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Msg {
    /// Navigation message.
    Nav(Nav),
}

impl Msg {
    /// Parses a u-blox message from a [`Frame`].
    pub fn from_frame(frame: &Frame) -> Result<Self, ()> {
        match frame.class {
            nav::Nav::CLASS => Ok(Self::Nav(Nav::from_frame(frame)?)),
            _ => Err(()),
        }
    }
}
