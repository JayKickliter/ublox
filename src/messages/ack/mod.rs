//! Ack/Nak Messages: i.e. Acknowledge or Reject messages to UBX-CFG
//! input messages.
//!
//! Messages in the UBX-ACK class output the
//! processing results to UBX-CFG and some other messages.

use crate::framing::Frame;
use crate::messages::Message;
use zerocopy::{AsBytes, ByteSlice, FromBytes, LayoutVerified, Unaligned};

/// Ack/Nak.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AckNak {
    Ack(Ack),
    Nak(Nak),
}

impl AckNak {
    /// ACK class.
    pub const CLASS: u8 = 0x05;

    /// Parses a Ack/Nak message from a [`Frame`].
    pub fn from_frame(frame: &Frame) -> Result<Self, ()> {
        if frame.class != Self::CLASS {
            return Err(());
        };

        match (frame.class, frame.id, frame.message.len()) {
            (Ack::CLASS, Ack::ID, Ack::LEN) => Ok(AckNak::Ack(Ack::parse(frame.message.as_ref())?)),
            (Nak::CLASS, Nak::ID, Nak::LEN) => Ok(AckNak::Nak(Nak::parse(frame.message.as_ref())?)),
            _ => Err(()),
        }
    }
}

/// Output upon processing of an input message.
///
/// A UBX-ACK-ACK is sent as soon as possible but at least within one second.
#[repr(C)]
#[derive(AsBytes, Clone, Debug, Eq, FromBytes, PartialEq, Unaligned)]
pub struct Ack {
    class: u8,
    id: u8,
}

impl Ack {
    /// Parses `Self` from provided buffer.
    pub fn parse<B>(bytes: B) -> Result<Self, ()>
    where
        B: ByteSlice,
    {
        Ok(LayoutVerified::<B, Self>::new(bytes).ok_or(())?.clone())
    }
}

impl Message for Ack {
    const CLASS: u8 = 0x05;
    const ID: u8 = 0x01;
    const LEN: usize = 2;
}

/// Output upon processing of an input message.
///
/// A UBX-ACK-NAK is sent as soon as possible but at least within one second.
#[repr(C)]
#[derive(AsBytes, Clone, Debug, Eq, FromBytes, PartialEq, Unaligned)]
pub struct Nak {
    class: u8,
    id: u8,
}

impl Nak {
    /// Parses `Self` from provided buffer.
    pub fn parse<B>(bytes: B) -> Result<Self, ()>
    where
        B: ByteSlice,
    {
        Ok(LayoutVerified::<B, Self>::new(bytes).ok_or(())?.clone())
    }
}

impl Message for Nak {
    const CLASS: u8 = 0x05;
    const ID: u8 = 0x00;
    const LEN: usize = 2;
}
