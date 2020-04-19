//! u-blox protocol framing and deframing state machines.

use crate::framing::{Checksum, Frame, FrameError, FrameVec};
use log::{trace, warn};

/// One-shot defamer utility function.
pub fn deframe<T>(bytes: T) -> Result<Option<Frame>, FrameError>
where
    T: IntoIterator<Item = u8>,
{
    let mut deframer = Deframer::new();
    for b in bytes {
        if let res @ Ok(Some(Frame { .. })) = deframer.push(b) {
            return res;
        }
    }
    Ok(None)
}

impl Deframer {
    /// Incrementally parses a u-blox message frame with the given
    /// `input`, returning a an error or optional [`Frame`].
    ///
    /// # Errors
    ///
    /// Upon any error when parsing the current `input` byte, this
    /// function returns an [`Error`].
    ///
    /// [`Frame`]: struct.Frame.html
    /// [`Error`]: enum.Error.html
    #[inline]
    pub fn push(&mut self, input: u8) -> Result<Option<Frame>, FrameError> {
        use self::Deframer::*;
        match self {
            Sync { accum, processed } => {
                const SYNCWORD: u16 = 0xB5_62;
                *accum = (*accum << 8) | u16::from(input);
                *processed += 1;
                if *accum == SYNCWORD {
                    *self = Deframer::Class;
                } else if *processed % 8 == 0 {
                    trace!("still searching for syncword after {} bytes", *processed);
                }
            }

            Class => {
                trace!("class {:#04x} ← sync", input);
                *self = Id {
                    cksum: Checksum::with(input),
                    class: input,
                }
            }

            Id { class, cksum } => {
                trace!("id {:#04x} ← class", input);
                *self = LengthLsb {
                    class: *class,
                    id: cksum.push(input),
                    cksum: *cksum,
                }
            }

            LengthLsb { class, id, cksum } => {
                trace!("len_l {:#04x} ← id", input);
                *self = LengthMsb {
                    class: *class,
                    id: *id,
                    len_b0: cksum.push(input),
                    cksum: *cksum,
                }
            }

            LengthMsb {
                class,
                id,
                len_b0,
                cksum,
            } => {
                let len = (usize::from(cksum.push(input)) << 8) | usize::from(*len_b0);
                // Revert to start state is len is larger than
                // unreasonable (and arbitrarily chosen) upper limit.
                if len > 999 {
                    warn!("declared message length {:#06x} is unreasonably large", len);
                    *self = Self::default();
                    return Ok(None);
                }
                trace!("len_h {:#04x} ← len_lsb", input);
                let message = FrameVec::with_capacity(len);
                *self = Message {
                    class: *class,
                    id: *id,
                    len,
                    message,
                    cksum: *cksum,
                }
            }

            Message {
                class,
                id,
                len,
                message,
                cksum,
            } => {
                message.push(cksum.push(input));
                if message.len() == *len {
                    *self = CkA {
                        class: *class,
                        id: *id,
                        message: message.clone(),
                        cksum_calc: cksum.take(),
                    };
                }
            }

            CkA {
                class,
                id,
                message,
                cksum_calc,
            } => {
                trace!("ck_a {:#04x} ← mesg", input);
                if input == cksum_calc.0 {
                    let mut msg = Vec::new();
                    ::std::mem::swap(message, &mut msg);
                    *self = CkB {
                        class: *class,
                        id: *id,
                        message: msg,
                        cksum_calc: *cksum_calc,
                    };
                } else {
                    warn!(
                        "ck_a mismatch, expected {:#04x}, got {:#04x}, msg {:02x?}",
                        cksum_calc.0, input, message
                    );
                    *self = Self::default();
                    return Err(FrameError::Checksum);
                }
            }

            CkB {
                class,
                id,
                message,
                cksum_calc,
            } => {
                trace!("ck_b {:#04x} ← ck_a", input);
                let mut msg = Vec::new();
                ::std::mem::swap(message, &mut msg);
                let ret = if input == cksum_calc.1 {
                    Ok(Some(Frame {
                        class: *class,
                        id: *id,
                        message: msg,
                    }))
                } else {
                    warn!(
                        "ck_b mismatch, expected {:#04x}, got {:#04x}, msg {:02x?}",
                        cksum_calc.1, input, msg
                    );
                    Err(FrameError::Checksum)
                };
                *self = Self::default();
                return ret;
            }
        };

        Ok(None)
    }

    /// Returns a new deframer.
    pub fn new() -> Self {
        Deframer::Sync {
            accum: 0,
            processed: 0,
        }
    }
}

impl Default for Deframer {
    fn default() -> Self {
        Self::new()
    }
}

/// A type for 'deframing' u-blox message frames.
#[derive(Debug, Clone)]
pub enum Deframer {
    /// Shift in every byte until matches value equals the syncword.
    #[doc(hidden)]
    Sync { accum: u16, processed: usize },

    /// No data, as the byte received durning this state is passed to
    /// next state.
    #[doc(hidden)]
    Class,

    /// Byte received during this state is passed to next state.
    #[doc(hidden)]
    Id { class: u8, cksum: Checksum },

    /// Length LSB received during this state is passed to next state.
    #[doc(hidden)]
    LengthLsb { class: u8, id: u8, cksum: Checksum },

    /// Collect length's MSB.
    #[doc(hidden)]
    LengthMsb {
        class: u8,
        id: u8,
        len_b0: u8,
        cksum: Checksum,
    },

    /// Push rx bytes into message until `message.len() == len`.
    #[doc(hidden)]
    Message {
        class: u8,
        id: u8,
        len: usize,
        message: FrameVec,
        cksum: Checksum,
    },

    /// Go to initial state if received byte doesnt match first byte
    /// of running checksum.
    #[doc(hidden)]
    CkA {
        class: u8,
        id: u8,
        message: FrameVec,
        cksum_calc: (u8, u8),
    },

    /// Go to initial state if received byte doesn't match second byte
    /// of running checksum.
    #[doc(hidden)]
    CkB {
        class: u8,
        id: u8,
        message: FrameVec,
        cksum_calc: (u8, u8),
    },
}

#[cfg(test)]
mod test {
    use super::Deframer;

    #[test]
    fn test_deframe() {
        let msg = [0xb5, 0x62, 0x05, 0x01, 0x01, 0x00, 0x06, 0x0d, 0x26];
        let mut deframer = Deframer::new();
        let mut res = None;
        for b in msg.as_ref() {
            res = deframer.push(*b).unwrap();
        }
        assert!(res.is_some());
    }
}
