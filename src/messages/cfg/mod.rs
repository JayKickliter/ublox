//! Configuration Input Messages: i.e. Configure the receiver.
//!
//! Messages in the CFG class can be used to configure the receiver and
//! poll current configuration values. Any messages in the CFG class sent
//! to the receiver are either acknowledged (with message UBX-ACK-ACK) if
//! processed successfully or rejected (with message UBX-ACK-NAK) if
//! processing unsuccessfully.

mod msg;
pub mod prt;
use crate::framing::Frame;
use crate::messages::Message;
pub use msg::SetMsgRates;

/// Configuration messages.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cfg {
    SetMsgRates(msg::SetMsgRates),
}

impl Cfg {
    /// CFG class.
    pub const CLASS: u8 = 0x06;

    /// Parses a configuration message from a [`Frame`].
    pub fn from_frame(frame: &Frame) -> Result<Self, ()> {
        if frame.class != Self::CLASS {
            return Err(());
        };

        match (frame.class, frame.id, frame.message.len()) {
            (msg::SetMsgRates::CLASS, msg::SetMsgRates::ID, msg::SetMsgRates::LEN) => Ok(
                Cfg::SetMsgRates(msg::SetMsgRates::parse(frame.message.as_ref())?),
            ),
            _ => Err(()),
        }
    }
}
