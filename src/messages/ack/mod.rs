//! Ack/Nak Messages: i.e. Acknowledge or Reject messages to UBX-CFG
//! input messages.
//!
//! Messages in the UBX-ACK class output the
//! processing results to UBX-CFG and some other messages.

use crate::framing::Frame;
use crate::messages::Message;

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
            (Ack::CLASS, Ack::ID, Ack::LEN) => {
                Ok(AckNak::Ack(Ack::deserialize(&mut frame.message.as_ref())?))
            }
            (Nak::CLASS, Nak::ID, Nak::LEN) => {
                Ok(AckNak::Nak(Nak::deserialize(&mut frame.message.as_ref())?))
            }
            _ => Err(()),
        }
    }
}

/// Output upon processing of an input message.
///
/// A UBX-ACK-ACK is sent as soon as possible but at least within one second.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ack {
    /// Acknowledged message's class.
    pub class: u8,
    /// Acknowledged message's ID.
    pub id: u8,
}

impl Message for Ack {
    const CLASS: u8 = 0x05;
    const ID: u8 = 0x01;
    const LEN: usize = 2;

    fn serialize<B: bytes::BufMut>(&self, dst: &mut B) -> Result<(), ()> {
        if dst.remaining_mut() < Self::LEN {
            return Err(());
        }

        dst.put_u8(self.class);
        dst.put_u8(self.id);

        Ok(())
    }

    fn deserialize<B: bytes::Buf>(src: &mut B) -> Result<Self, ()> {
        if src.remaining() < Self::LEN {
            return Err(());
        }

        let class = src.get_u8();
        let id = src.get_u8();

        Ok(Self { class, id })
    }
}

/// Output upon processing of an input message.
///
/// A UBX-ACK-NAK is sent as soon as possible but at least within one second.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nak {
    class: u8,
    id: u8,
}

impl Message for Nak {
    const CLASS: u8 = 0x05;
    const ID: u8 = 0x00;
    const LEN: usize = 2;

    fn serialize<B: bytes::BufMut>(&self, dst: &mut B) -> Result<(), ()> {
        if dst.remaining_mut() < Self::LEN {
            return Err(());
        }

        dst.put_u8(self.class);
        dst.put_u8(self.id);

        Ok(())
    }

    fn deserialize<B: bytes::Buf>(src: &mut B) -> Result<Self, ()> {
        if src.remaining() < Self::LEN {
            return Err(());
        }

        let class = src.get_u8();
        let id = src.get_u8();

        Ok(Self { class, id })
    }
}
