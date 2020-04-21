//! Port configuration messages.

use crate::messages::{primitive::*, Message};
use bitfield::bitfield;

/// Port configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Prt {
    /// Port configuration for UART ports
    ///
    /// Note that this message can affect baud rate and other
    /// transmission parameters. Because there may be messages queued
    /// for transmission there may be uncertainty about which protocol
    /// applies to such messages. In addition a message currently in
    /// transmission may be corrupted by a protocol change. Host data
    /// reception parameters may have to be changed to be able to
    /// receive future messages, including the acknowledge message
    /// resulting from the CFG-PRT message.
    Uart {
        /// TX ready PIN configuration.
        tx_ready: TxReady,
        /// A bit mask describing the UART mode.
        mode: UartMode,
        /// Baud rate in bits/second.
        baud_rate: U4,
        /// A mask describing which input protocols are active.
        ///
        /// Each bit of this mask is used for a protocol. Through
        /// that, multiple protocols can be defined on a single port.
        in_proto_mask: InProtoMask,
        /// A mask describing which output protocols are active.
        ///
        /// Each bit of this mask is used for a protocol. Through that,
        /// multiple protocols can be defined on a single port.
        out_proto_mask: OutProtoMask,
        /// Flags bit mask
        flags: Flags,
    },
    /// Port configuration for I2C (DDC) port.
    I2c {
        /// TX ready PIN configuration.
        tx_ready: TxReady,
        ///  I2C (DDC) Mode Flags
        mode: I2cMode,
        /// A mask describing which input protocols are active.
        ///
        /// Each bit of this mask is used for a protocol. Through
        /// that, multiple protocols can be defined on a single port.
        in_proto_mask: InProtoMask,
        /// A mask describing which output protocols are active.
        ///
        /// Each bit of this mask is used for a protocol. Through that,
        /// multiple protocols can be defined on a single port.
        out_proto_mask: OutProtoMask,
        /// Flags bit mask
        flags: Flags,
    },
    /// Port configuration for SPI port.
    Spi {
        /// TX ready PIN configuration.
        tx_ready: TxReady,
        ///  SPI Mode Flags
        mode: SpiMode,
        /// A mask describing which input protocols are active.
        ///
        /// Each bit of this mask is used for a protocol. Through
        /// that, multiple protocols can be defined on a single port.
        in_proto_mask: InProtoMask,
        /// A mask describing which output protocols are active.
        ///
        /// Each bit of this mask is used for a protocol. Through that,
        /// multiple protocols can be defined on a single port.
        out_proto_mask: OutProtoMask,
        /// Flags bit mask
        flags: Flags,
    },
}

impl Prt {
    const I2C_PORT: u8 = 0;
    const UART_PORT: u8 = 1;
    const SPI_PORT: u8 = 4;
}

impl Message for Prt {
    const CLASS: u8 = 0x06;
    const ID: u8 = 0x00;
    const LEN: usize = 20;

    fn to_bytes(&self, dst: &mut [u8]) -> Result<(), ()> {
        use byteorder::{WriteBytesExt, LE};
        use std::io::Cursor;

        let mut csr = Cursor::new(dst);

        match self {
            Prt::Uart {
                tx_ready,
                mode,
                baud_rate,
                in_proto_mask,
                out_proto_mask,
                flags,
            } => {
                csr.write_u8(Self::UART_PORT).map_err(|_| ())?;
                // reserved 1
                csr.write_u8(0).map_err(|_| ())?;
                csr.write_u16::<LE>(tx_ready.0).map_err(|_| ())?;
                csr.write_u32::<LE>(mode.0).map_err(|_| ())?;
                csr.write_u32::<LE>(*baud_rate).map_err(|_| ())?;
                csr.write_u16::<LE>(in_proto_mask.0).map_err(|_| ())?;
                csr.write_u16::<LE>(out_proto_mask.0).map_err(|_| ())?;
                csr.write_u16::<LE>(flags.0).map_err(|_| ())?;
                // reserved2
                csr.write_u16::<LE>(0).map_err(|_| ())?;
            }
            Prt::I2c {
                tx_ready,
                mode,
                in_proto_mask,
                out_proto_mask,
                flags,
            } => {
                csr.write_u8(Self::I2C_PORT).map_err(|_| ())?;
                // reserved 1
                csr.write_u8(0).map_err(|_| ())?;
                csr.write_u16::<LE>(tx_ready.0).map_err(|_| ())?;
                csr.write_u32::<LE>(mode.0).map_err(|_| ())?;
                // reserved2
                csr.write_u32::<LE>(0).map_err(|_| ())?;
                csr.write_u16::<LE>(in_proto_mask.0).map_err(|_| ())?;
                csr.write_u16::<LE>(out_proto_mask.0).map_err(|_| ())?;
                csr.write_u16::<LE>(flags.0).map_err(|_| ())?;
                // reserved3
                csr.write_u16::<LE>(0).map_err(|_| ())?;
            }
            Prt::Spi {
                tx_ready,
                mode,
                in_proto_mask,
                out_proto_mask,
                flags,
            } => {
                csr.write_u8(Self::SPI_PORT).map_err(|_| ())?;
                // reserved 1
                csr.write_u8(0).map_err(|_| ())?;
                csr.write_u16::<LE>(tx_ready.0).map_err(|_| ())?;
                csr.write_u32::<LE>(mode.0).map_err(|_| ())?;
                // reserved2
                csr.write_u32::<LE>(0).map_err(|_| ())?;
                csr.write_u16::<LE>(in_proto_mask.0).map_err(|_| ())?;
                csr.write_u16::<LE>(out_proto_mask.0).map_err(|_| ())?;
                csr.write_u16::<LE>(flags.0).map_err(|_| ())?;
                // reserved3
                csr.write_u16::<LE>(0).map_err(|_| ())?;
            }
        }
        assert_eq!(csr.position() as usize, Self::LEN);
        Ok(())
    }
}

bitfield! {
    /// TX ready pin configuration.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct TxReady(X2);
    impl Debug;
    /// Threshold
    ///
    /// The given threshold is multiplied by 8 bytes.
    ///
    /// The TX ready PIN goes active after >= thres*8 bytes are
    /// pending for the port and going inactive after the last pending
    /// bytes have been written to hardware (0-4 bytes before end of
    /// stream).
    ///
    /// - 0x000 no threshold
    /// - 0x001 8byte
    /// - 0x002 16byte
    /// - ...
    /// - 0x1FE 4080byte
    /// - 0x1FF 4088byte
    pub thres, set_thres: 15, 7;
    /// PIO to be used (must not be in use by another function)
    pub pin, set_pin: 6, 2;
    /// Polarity
    ///
    /// - 0 High-active
    /// - 1 Low-active
    pub pol, set_pol: 1;
    /// Enable TX ready feature for this port
    pub en, set_en: 0;
}

bitfield! {
    /// Bitfield `mode` for uart port configuration.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct UartMode(X4);
    impl Debug;
    /// Number of Stop bits
    ///
    /// - 00 1 Stop bit
    /// - 01 1.5 Stop bit
    /// - 10 2 Stop bit
    /// - 11 0.5 Stop bit
    pub n_stop_bits, set_n_stop_bits: 13, 12;
    /// Parity
    ///
    /// - 000 Even parity
    /// - 001 Odd parity
    /// - 10X No parity
    /// - X1X Reserved
    pub parity, set_parity: 11, 9;
    /// Character length
    ///
    /// - 00 5bit (not supported)
    /// - 01 6bit (not supported)
    /// - 10 7bit (supported only with parity)
    /// - 11 8bit
    pub char_len, set_char_len: 7, 6;
}

bitfield! {
    /// Bitfield `mode` for i2c port configuration.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct I2cMode(X4);
    impl Debug;
    u8;
    /// Slave addr.
    pub slave_addr, set_slave_addr: 7, 1;
}

bitfield! {
    /// Bitfield `mode` for spi port configuration.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct SpiMode(X4);
    impl Debug;
    u8;
    /// Number of bytes containing 0xFF to receive before switching off reception.
    ///
    /// Range: 0 (mechanism off) - 63
    pub ff_cnt, set_ff_cnt: 13, 8;
    /// Phase.
    ///
    /// - 00 SPI Mode 0: CPOL = 0, CPHA = 0
    /// - 01 SPI Mode 1: CPOL = 0, CPHA = 1
    /// - 10 SPI Mode 2: CPOL = 1, CPHA = 0
    /// - 11 SPI Mode 3: CPOL = 1, CPHA = 1
    pub spi_mode, set_spi_mode: 2, 1;
}

bitfield! {
    /// A mask describing which input protocols are active.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct InProtoMask(X2);
    impl Debug;
    /// RTCM3 protocol (not supported in protocol versions less than 20)
    pub in_rtcm3, set_in_rtcm3: 5;
    /// RTCM2 protocol
    pub in_rtcm, set_in_rtcm: 2;
    /// NMEA protocol
    pub in_nmea, set_in_nmea: 1;
    /// UBX protocol
    pub in_ubx, set_in_ubx: 0;
}

bitfield! {
    /// A mask describing which output protocols are active.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct OutProtoMask(X2);
    impl Debug;
    /// RTCM3 protocol (not supported in protocol versions less than 20)
    pub out_rtcm3, set_out_rtcm3: 5;
    /// NMEA protocol
    pub out_nmea, set_out_nmea: 1;
    /// UBX protocol
    pub out_ubx, set_out_ubx: 0;
}

bitfield! {
    /// A mask describing which output protocols are active.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct Flags(X2);
    impl Debug;
    /// Extended TX timeout
    ///
    /// Tf set, the port will time out if allocated TX memory >=4 kB
    /// and no activity for 1.5 s. If not set the port will time out
    /// if no activity for 1.5 s regardless on the amount of allocated
    /// TX memory.
    pub extended_tx_timeout, set_extended_tx_timeout: 1;
}
