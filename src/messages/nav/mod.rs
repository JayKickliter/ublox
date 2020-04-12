//! Navigation messages.

mod timegps;
pub use self::timegps::*;
use crate::framing::Frame;
use nom::{alt, do_parse, named_attr, tag};

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
}

impl Nav {
    /// NAV class.
    pub const CLASS: u8 = 0x01;

    /// Parses a navigation message from a [`Frame`].
    pub fn from_frame(frame: &Frame) -> Result<Self, ()> {
        if frame.class != Self::CLASS {
            return Err(());
        };

        match frame.id {
            TimeGps::ID => Ok(Self::TimeGps(time_gps(&frame.message).map_err(|_| ())?.1)),
            _ => Err(()),
        }
    }
}

named_attr!(
    #[allow(missing_docs)],
    pub navmsg<&[u8], Nav>,
    do_parse!(tag!([0x01]) >>
              navmsg: alt!(
                  time_gps => { | msg | Nav::TimeGps(msg) }
              ) >>
              (navmsg)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nav_timegps() {
        let msg = vec![0x01, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        navmsg(&msg).unwrap();
    }
}
