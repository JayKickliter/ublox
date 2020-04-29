//! Navigation messages.

mod pvt;
mod timegps;
pub use self::pvt::*;
pub use self::timegps::*;
use crate::framing::Frame;
use crate::messages::Message;

/// Navigation Results Messages
///
/// Includes:
/// - Position
/// - Speed
/// - Time
/// - Acceleration
/// - Heading
/// - DOP
/// - SVs used
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Nav {
    TimeGps(TimeGps),
    Pvt(Pvt),
}

impl Nav {
    /// NAV class.
    pub const CLASS: u8 = 0x01;

    /// Parses a navigation message from a [`Frame`].
    pub fn from_frame(frame: &Frame) -> Result<Self, ()> {
        if frame.class != Self::CLASS {
            return Err(());
        };

        match (frame.class, frame.id, frame.message.len()) {
            (TimeGps::CLASS, TimeGps::ID, TimeGps::LEN) => Ok(Nav::TimeGps(TimeGps::deserialize(
                &mut frame.message.as_slice(),
            )?)),
            (Pvt::CLASS, Pvt::ID, Pvt::LEN) => {
                Ok(Nav::Pvt(Pvt::deserialize(&mut frame.message.as_slice())?))
            }
            _ => Err(()),
        }
    }
}
