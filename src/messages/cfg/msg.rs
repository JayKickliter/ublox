use crate::messages::{primitive::*, Message};
use zerocopy::{AsBytes, ByteSlice, FromBytes, LayoutVerified, Unaligned};

/// Get/set message rate configuration(s) to/from the receiver.
///
/// Send rate is relative to the event a message is registered on. For
/// example, if the rate of a navigation message is set to 2, the
/// message is sent every second navigation solution.
#[repr(C)]
#[derive(AsBytes, Clone, Debug, Eq, FromBytes, PartialEq, Unaligned)]
pub struct SetMsgRates {
    /// Message class of message to configure (not `Self`'s class).
    pub class: U1,
    /// Message identifier of message to configure (not `Self`'s identifier).
    pub id: U1,
    /// DDC (I²C) rate.
    pub ddc: U1,
    /// UART 1 rate.
    pub uart1: U1,
    /// USB rate.
    pub usb: U1,
    /// SPI rate.
    pub spi: U1,
    /// Reserved
    pub reserved1: U1,
    /// Reserved
    pub reserved2: U1,
}

impl Message for SetMsgRates {
    const CLASS: u8 = 0x06;
    const ID: u8 = 0x01;
    const LEN: usize = 8;

    fn to_bytes(&self, buf: &mut [u8]) -> Result<(), ()> {
        if buf.len() < Self::LEN {
            return Err(());
        };
        buf[..Self::LEN].clone_from_slice(self.as_bytes());
        unimplemented!("this message type doesn't work as expected")
    }
}

impl SetMsgRates {
    /// Parses `Self` from provided buffer.
    pub fn parse<B>(bytes: B) -> Result<Self, ()>
    where
        B: ByteSlice,
    {
        Ok(LayoutVerified::<B, Self>::new(bytes).ok_or(())?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse() {
        let bytes = [0x00, 0x01, 0x20, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(
            SetMsgRates::parse(bytes[1..].as_ref()).unwrap(),
            SetMsgRates {
                class: 0x01,
                id: 0x20,
                ddc: 0x00,
                uart1: 0x01,
                usb: 0x01,
                spi: 0x00,
                reserved1: 0x00,
                reserved2: 0x00,
            }
        )
    }

    #[test]
    fn test_can_encode() {
        let bytes = [0x01, 0x20, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00];
        let msg = SetMsgRates {
            class: 0x01,
            id: 0x20,
            ddc: 0x00,
            uart1: 0x01,
            usb: 0x01,
            spi: 0x00,
            reserved1: 0x00,
            reserved2: 0x00,
        };

        assert_eq!(msg.as_bytes(), bytes.as_ref());
    }
}
