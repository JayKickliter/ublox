use crate::error::Result;
use i2c_linux::{I2c, Message as I2cMessage, ReadFlags, WriteFlags};
use std::thread;
use std::{fmt::Debug, fs::File, path::Path, time::Duration};
use sysfs_gpio as gpio;
use ublox::{framing::Deframer, messages::Msg};
use ublox::{
    framing::{frame, Frame},
    messages::{cfg, nav, Message},
};

pub fn i2c_loop<P: AsRef<Path> + Debug>(path: &P, addr: u16, tx_ready_pin: Option<u64>) -> Result {
    let mut dev = I2c::from_path(path)?;
    let mut deframer = Deframer::new();
    let mut scratch = [0x00_u8; 128];

    // Disable all protocols on UART
    {
        use cfg::prt;
        let msg = prt::Prt::Uart {
            tx_ready: prt::TxReady(0),
            in_proto_mask: {
                let mut mask = prt::InProtoMask(0);
                mask.set_in_ubx(true);
                mask
            },
            out_proto_mask: prt::OutProtoMask(0),
            baud_rate: 9600,
            flags: prt::Flags(0),
            mode: {
                let mut mode = prt::UartMode(0);
                mode.set_n_stop_bits(0b00);
                mode.set_parity(0b100);
                mode.set_char_len(0b11);
                mode
            },
        };
        let len = frame(&msg, &mut scratch).unwrap();
        log::debug!("{:02x?}", &scratch[..len]);
        write(&mut dev, addr, &scratch[..len])?;
    }

    // Configure I2C port to be ubx protocol only.
    {
        use cfg::prt;
        let msg = prt::Prt::I2c {
            tx_ready: {
                let mut txr = prt::TxReady(0);
                txr.set_thres(1);
                txr.set_pin(13);
                txr.set_en(true);
                txr
            },
            mode: {
                let mut mode = prt::I2cMode(0);
                mode.set_slave_addr(addr as u8);
                mode
            },
            in_proto_mask: {
                let mut mask = prt::InProtoMask(0);
                mask.set_in_ubx(true);
                mask
            },
            out_proto_mask: {
                let mut mask = prt::OutProtoMask(0);
                mask.set_out_ubx(true);
                mask
            },
            flags: prt::Flags(0),
        };
        let len = frame(&msg, &mut scratch).unwrap();
        log::debug!("{:02x?}", &scratch[..len]);
        write(&mut dev, addr, &scratch[..len])?;
    }

    {
        let frm = Frame {
            class: 6,
            id: 1,
            message: vec![nav::Pvt::CLASS, nav::Pvt::ID, 1],
        };
        let en_msg = frm.into_framed_vec();
        log::debug!("{:x?}", en_msg);
        write(&mut dev, addr, &en_msg)?;
    }

    {
        let frm = Frame {
            class: 6,
            id: 1,
            message: vec![nav::TimeGps::CLASS, nav::TimeGps::ID, 1],
        };
        let en_msg = frm.into_framed_vec();
        log::debug!("{:x?}", en_msg);
        write(&mut dev, addr, &en_msg)?;
    }

    let mut pin: Option<(gpio::Pin, gpio::PinPoller)> = tx_ready_pin.map(|pinnum| {
        let pin = gpio::Pin::new(pinnum);
        pin.export().expect("GPIO pin does can not be exported");
        pin.set_direction(gpio::Direction::In)
            .expect("GPIO pin does can not be an input");
        pin.set_edge(gpio::Edge::RisingEdge)
            .expect("GPIO pin does not support interrupts");
        (
            pin,
            pin.get_poller().expect("GPIO pin does not support polling"),
        )
    });

    loop {
        if let Some((pin, poller)) = pin.as_mut() {
            if 0 == pin.get_value().unwrap() {
                const TIMEOUT: isize = 1100;
                match poller.poll(TIMEOUT) {
                    Err(e) => log::error!("polling tx_ready {} ", e),
                    Ok(None) => log::warn!("timed out after waiting {} ms for tx_ready", TIMEOUT),
                    Ok(Some(_)) => log::info!("tx_ready"),
                }
            }
        };

        let mut n_avail;

        // The `Number of Bytes available (High Byte)` register (`0xFD`) is sometimes glitchy.
        // Give it a few tries to think about what it did.
        //
        // NOTE: when it does glitch the upper most nibble seems to always be `0x8`, e.g.
        //
        // ```
        // n_avail 0     0000
        // n_avail 32768 8000 is too high, retry
        // n_avail 0     0000
        // ```
        loop {
            n_avail = available(&mut dev, addr)?;
            if n_avail != 0x8000 && n_avail != 0x0080 {
                break;
            }
            log::warn!(
                "n_avail {} {:#06x} appears to be a glitch, retry",
                n_avail,
                n_avail
            );
            thread::sleep(Duration::from_millis(50));
        }
        thread::sleep(Duration::from_millis(50));

        if n_avail == 0 {
            thread::sleep(Duration::from_millis(200));
            continue;
        }

        log::debug!("n_avail {} {:#06x}", n_avail, n_avail);

        let read_len = usize::min(n_avail, scratch.len());
        let read_buf = &mut scratch[..read_len];
        if read(&mut dev, addr, read_buf).is_err() {
            log::error!("i2c read error, trying once more");
            continue;
        }

        for &mut b in read_buf {
            match deframer.push(b) {
                None => (),
                Some(frame) => match Msg::from_frame(&frame) {
                    Err(_) => log::warn!("unhandled frame: {:?}", frame),
                    Ok(msg) => println!("\n{:?}\n", msg),
                },
            }
        }
    }
}

fn available(dev: &mut I2c<File>, addr: u16) -> Result<usize> {
    const UBX_BYTES_AVAIL_REG: u8 = 0xFD;
    let mut available = [0; 2];
    let mut msgs = [
        I2cMessage::Write {
            address: addr,
            data: &[UBX_BYTES_AVAIL_REG],
            flags: WriteFlags::default(),
        },
        I2cMessage::Read {
            address: addr,
            data: &mut available,
            flags: ReadFlags::default(),
        },
    ];
    if dev.i2c_transfer(&mut msgs).is_err() {
        log::error!("i2c transfer failure. trying once more");
        dev.i2c_transfer(&mut msgs)?;
    }
    Ok(u16::from_be_bytes(available).into())
}

fn read(dev: &mut I2c<File>, addr: u16, dst: &mut [u8]) -> Result {
    const UBX_WRITE_REG: u8 = 0xFF;
    let dst_len = dst.len();
    let mut msgs = [
        I2cMessage::Write {
            address: addr,
            data: &[UBX_WRITE_REG],
            flags: WriteFlags::default(),
        },
        I2cMessage::Read {
            address: addr,
            data: dst,
            flags: ReadFlags::default(),
        },
    ];
    dev.i2c_transfer(&mut msgs)?;
    if let I2cMessage::Read { data: read, .. } = &msgs[1] {
        assert_eq!(read.len(), dst_len);
    }
    Ok(())
}

fn write(dev: &mut I2c<File>, addr: u16, src: &[u8]) -> Result {
    const UBX_WRITE_REG: u8 = 0xFF;
    let mut msgs = [
        I2cMessage::Write {
            address: addr,
            data: &[UBX_WRITE_REG],
            flags: WriteFlags::default(),
        },
        I2cMessage::Write {
            address: addr,
            data: &src,
            flags: WriteFlags::default(),
        },
    ];
    if dev.i2c_transfer(&mut msgs).is_err() {
        log::error!("i2c transfer failure. trying once more");
        dev.i2c_transfer(&mut msgs)?;
    }
    if let I2cMessage::Write { data: msg_src, .. } = msgs[1] {
        assert_eq!(msg_src.len(), src.len());
    }
    Ok(())
}
