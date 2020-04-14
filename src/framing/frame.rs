use crate::framing::{Checksum, FrameVec};
use crate::messages::Message;

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
    /// The message's message.
    ///
    /// `message` is just a buffer of bytes and can be dispatched to
    /// an appropriate message-specific parser based on `class` and
    /// `id`.
    pub message: FrameVec,
}

impl Frame {
    /// Converts `Frame` into to framed vector of bytes.
    pub fn into_framed_vec(self) -> FrameVec {
        let Frame {
            class,
            id,
            mut message,
        } = self;
        // Prepend frame data to message by first appending it, then
        // rotating it to the front.
        {
            let [len_lsb, len_msb] = (message.len() as u16).to_le_bytes();
            let prefix = [0xB5, 0x62, class, id, len_lsb, len_msb];
            message.extend_from_slice(&prefix);
            message.rotate_right(prefix.len());
        }
        // Append checksum.
        {
            let mut cksm = Checksum::default();
            // The checksum is calculated from class to end of message, hence
            // `skip(2)`
            for b in message.iter().skip(2) {
                cksm.push(*b);
            }
            let (ck_a, ck_b) = cksm.take();
            message.push(ck_a);
            message.push(ck_b);
        }
        message
    }
}

/// Frame a u-blox message to a buffer.
pub fn frame<M: Message>(msg: &M, buf: &mut [u8]) -> Result<usize, ()> {
    const FRAME_OVERHEAD: usize = 8;
    if buf.len() < (FRAME_OVERHEAD + M::LEN) {
        return Err(());
    }
    let buf = &mut buf[..M::LEN + FRAME_OVERHEAD];
    // Prelude
    {
        let [len_lsb, len_msb] = (M::LEN as u16).to_le_bytes();
        buf[..6].clone_from_slice(&[0xB5, 0x62, M::CLASS, M::ID, len_lsb, len_msb]);
    }
    // Mesage body.
    msg.to_bytes(buf[6..(M::LEN + 6)].as_mut())?;
    // Append checksum.
    {
        let mut cksm = Checksum::default();
        // The checksum is calculated from class to end of message, hence
        // `skip(2)`
        for b in buf.iter().skip(2) {
            cksm.push(*b);
        }
        let (ck_a, ck_b) = cksm.take();
        buf[M::LEN + 6..].clone_from_slice(&[ck_a, ck_b]);
    }
    Ok(M::LEN + FRAME_OVERHEAD)
}
