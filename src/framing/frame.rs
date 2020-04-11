use crate::framing::{Checksum, FrameVec};

/// The type returned by [`Deframer::push()`] upon successfully parsing
/// a u-blox message.
///
/// [`Deframer::push()`]: enum.Deframer.html#method.push
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    /// Message class.
    pub class: u8,
    /// Message ID.
    ///
    /// Message ID's are not globally unique, but they are unique per
    /// `class`.
    pub id: u8,
    /// The message's payload.
    ///
    /// `payload` is just a buffer of bytes and can be dispatched to
    /// an appropriate message-specific parser based on `class` and
    /// `id`.
    pub payload: FrameVec,
}

impl Frame {
    /// Converts `Frame` into to framed vector of bytes.
    pub fn into_framed_vec(self) -> FrameVec {
        let Frame {
            class,
            id,
            mut payload,
        } = self;
        // Prepend frame data to message by first appending it, then
        // rotating it to the front.
        {
            let [len_lsb, len_msb] = (payload.len() as u16).to_le_bytes();
            let prefix = [0xB5, 0x62, class, id, len_lsb, len_msb];
            payload.extend_from_slice(&prefix);
            payload.rotate_right(prefix.len());
        }
        // Append checksum.
        {
            let mut cksm = Checksum::default();
            // The checksum is calculated from class to end of payload, hence
            // `skip(2)`
            for b in payload.iter().skip(2) {
                cksm.push(*b);
            }
            let (ck_a, ck_b) = cksm.take();
            payload.push(ck_a);
            payload.push(ck_b);
        }
        payload
    }
}
