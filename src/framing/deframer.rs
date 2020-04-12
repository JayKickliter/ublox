//! u-blox protocol framing and deframing state machines.

use crate::framing::{Checksum, Frame, FrameError, FrameVec};

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
            Sync(accum) => {
                const SYNCWORD: u16 = 0xB5_62;
                *accum = (*accum << 8) | u16::from(input);
                if *accum == SYNCWORD {
                    *self = Deframer::Class;
                }
            }

            Class => {
                *self = Id {
                    cksum: Checksum::with(input),
                    class: input,
                }
            }

            Id { class, cksum } => {
                *self = LengthLsb {
                    class: *class,
                    id: cksum.push(input),
                    cksum: *cksum,
                }
            }

            LengthLsb { class, id, cksum } => {
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
                let payload = FrameVec::with_capacity(len);
                *self = Payload {
                    class: *class,
                    id: *id,
                    len,
                    payload,
                    cksum: *cksum,
                }
            }

            Payload {
                class,
                id,
                len,
                payload,
                cksum,
            } => {
                payload.push(cksum.push(input));
                if payload.len() == *len {
                    *self = CkA {
                        class: *class,
                        id: *id,
                        payload: payload.clone(),
                        cksum_calc: cksum.take(),
                    };
                }
            }

            CkA {
                class,
                id,
                payload,
                cksum_calc,
            } => {
                if input == cksum_calc.0 {
                    let mut pay = Vec::new();
                    ::std::mem::swap(payload, &mut pay);
                    *self = CkB {
                        class: *class,
                        id: *id,
                        payload: pay,
                        cksum_calc: *cksum_calc,
                    };
                } else {
                    *self = Self::default();
                    return Err(FrameError::Checksum);
                }
            }

            CkB {
                class,
                id,
                payload,
                cksum_calc,
            } => {
                let mut pay = Vec::new();
                ::std::mem::swap(payload, &mut pay);
                let ret = if input == cksum_calc.1 {
                    Ok(Some(Frame {
                        class: *class,
                        id: *id,
                        payload: pay,
                    }))
                } else {
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
        Deframer::Sync(0)
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
    Sync(u16),

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

    /// Push rx bytes into payload until `payload.len() == len`.
    #[doc(hidden)]
    Payload {
        class: u8,
        id: u8,
        len: usize,
        payload: FrameVec,
        cksum: Checksum,
    },

    /// Go to initial state if received byte doesnt match first byte
    /// of running checksum.
    #[doc(hidden)]
    CkA {
        class: u8,
        id: u8,
        payload: FrameVec,
        cksum_calc: (u8, u8),
    },

    /// Go to initial state if received byte doesn't match second byte
    /// of running checksum.
    #[doc(hidden)]
    CkB {
        class: u8,
        id: u8,
        payload: FrameVec,
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
