//! u-blox protocol framing and deframing state machines.

/// TODO: add `std` feature and use `heapless::Vec<u8,
/// heapless::consts::U128>` when not `std` feature is not enabled.
type DeframeVec = Vec<u8>;

/// A type for 'deframing' u-blox message frames.
#[derive(Debug)]
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
        payload: DeframeVec,
        cksum: Checksum,
    },

    /// Go to initial state if received byte doesnt match first byte
    /// of running checksum.
    #[doc(hidden)]
    CkA {
        class: u8,
        id: u8,
        payload: DeframeVec,
        cksum_calc: (u8, u8),
    },

    /// Go to initial state if received byte doesn't match second byte
    /// of running checksum.
    #[doc(hidden)]
    CkB {
        class: u8,
        id: u8,
        payload: DeframeVec,
        cksum_calc: (u8, u8),
    },
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
                let payload = DeframeVec::with_capacity(len);
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
                    return Err(FrameError::Crc);
                } else {
                    *self = Self::default();
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
                    Err(FrameError::Crc)
                };
                *self = Self::default();
                return ret;
            }
        };

        Ok(None)
    }
}

/// The type returned by [`Deframer::push()`] upon successfully parsing
/// a u-blox message.
///
/// [`Deframer::push()`]: enum.Deframer.html#method.push
#[derive(Debug)]
pub struct Frame {
    /// Message class.
    class: u8,
    /// Message ID.
    ///
    /// Message ID's are not globally unique, but they are unique per
    /// `class`.
    id: u8,
    /// The message's payload.
    ///
    /// `payload` is just a buffer of bytes and can be dispatched to
    /// an appropriate message-specific parser based on `class` and
    /// `id`.
    payload: DeframeVec,
}

impl Default for Deframer {
    fn default() -> Self {
        Deframer::Sync(0)
    }
}

/// A type used for incrementally calculating u-blox protocol message
/// checksums.
#[derive(Debug, Default, Clone, Copy)]
pub struct Checksum(Option<(u8, u8)>);

impl Checksum {
    /// Returns self initialized with the first byte.
    ///
    /// As this initializes `self`, you must not call `push()` with
    /// this same byte again, else the calculated CRC will be
    /// incorrect.
    pub fn with(input: u8) -> Self {
        let mut s = Self::default();
        s.push(input);
        s
    }

    /// Update the running checksum with a received byte.
    #[inline]
    fn push(&mut self, input: u8) -> u8 {
        let (ck_a, ck_b) = self.0.get_or_insert((0, 0));
        *ck_a = ck_a.wrapping_add(input);
        *ck_b = ck_b.wrapping_add(*ck_a);
        input
    }

    /// Returns the running calculated checksum bytes in tuple
    /// consisting of `(ck_a, ck_b)`.
    fn take(&mut self) -> (u8, u8) {
        self.0.take().unwrap_or((0, 0))
    }
}

/// The error type returned by [`Deframer::push()`].
///
/// [`Deframer::push()`]: enum.Deframer.html#method.push
#[derive(Debug)]
pub enum FrameError {
    /// The payload length parsed out of message is larger than we can
    /// store.
    #[cfg(not(feature = "std"))]
    Size {
        /// Declared message length parsed from byte stream.
        declared: usize,
        /// Payload buffer's capacity.
        capacity: usize,
    },

    /// CRC mismatch.
    ///
    /// Note that declared or calaculated CRCs are *not* included with
    /// the error. This is because the defamer may return this error
    /// after receiving only the first declared CRC byte.
    Crc,
}
