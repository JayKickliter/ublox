use crate::messages::{primitive::*, Message};

/// Get/set message rate configuration(s) to/from the receiver.
///
/// Send rate is relative to the event a message is registered on. For
/// example, if the rate of a navigation message is set to 2, the
/// message is sent every second navigation solution.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    // reserved1: U1,
    // reserved2: U1,
}

impl Message for SetMsgRates {
    const CLASS: u8 = 0x06;
    const ID: u8 = 0x01;
    const LEN: usize = 8;

    fn serialize<B: bytes::BufMut>(&self, dst: &mut B) -> Result<(), ()> {
        if dst.remaining_mut() < Self::LEN {
            return Err(());
        };

        let &Self {
            class,
            id,
            ddc,
            uart1,
            usb,
            spi,
        } = self;

        dst.put_u8(class);
        dst.put_u8(id);
        dst.put_u8(ddc);
        dst.put_u8(uart1);
        dst.put_u8(usb);
        dst.put_u8(spi);
        // Reserved 1
        dst.put_u8(0);
        // Reserved 2
        dst.put_u8(0);

        Ok(())
    }

    fn deserialize<B: bytes::Buf>(src: &mut B) -> Result<Self, ()> {
        if src.remaining() < Self::LEN {
            return Err(());
        }

        let class = src.get_u8();
        let id = src.get_u8();
        let ddc = src.get_u8();
        let uart1 = src.get_u8();
        let usb = src.get_u8();
        let spi = src.get_u8();
        let _reserved1 = src.get_u8();
        let _reserved2 = src.get_u8();

        Ok(Self {
            class,
            id,
            ddc,
            uart1,
            usb,
            spi,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse() {
        let bytes = [0x00, 0x01, 0x20, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(
            SetMsgRates::deserialize(&mut &bytes[1..]).unwrap(),
            SetMsgRates {
                class: 0x01,
                id: 0x20,
                ddc: 0x00,
                uart1: 0x01,
                usb: 0x01,
                spi: 0x00,
            }
        )
    }

    #[test]
    fn test_can_encode() {
        let bytes = [0x01_u8, 0x20, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00];
        let msg = SetMsgRates {
            class: 0x01,
            id: 0x20,
            ddc: 0x00,
            uart1: 0x01,
            usb: 0x01,
            spi: 0x00,
        };

        assert_eq!(msg, SetMsgRates::deserialize(&mut &bytes[..]).unwrap());
    }
}
