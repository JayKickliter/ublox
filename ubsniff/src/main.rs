mod cmdline;
use cmdline::Cmdline;
use log::{debug, error, warn};
#[cfg(target_os = "linux")]
use std::fmt::Debug;
use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{BufReader, ErrorKind, Read},
    path::Path,
    time::Duration,
};
use structopt::StructOpt;
use ublox::{framing::Deframer, messages::Msg};

type Result<T = ()> = ::std::result::Result<T, Box<dyn Error>>;

fn main() {
    let cmdline = Cmdline::from_args();
    env_logger::init();
    let res = match cmdline {
        Cmdline::File { path } => file_loop(&path),
        #[cfg(target_os = "linux")]
        Cmdline::I2c { path, addr } => i2c_loop(&path, addr),
        Cmdline::Serial { path, baud } => uart_loop(&path, baud),
    };
    if let Err(e) = res {
        error!("exiting early with {}", e);
        ::std::process::exit(1);
    }
}

fn file_loop(path: &Path) -> Result {
    let file = File::open(path)?;

    let mut deframer = Deframer::new();
    for b in file.bytes() {
        match deframer.push(b?) {
            Err(e) => eprintln!("deframe error {:?}", e),
            Ok(None) => (),
            Ok(Some(frame)) => match Msg::from_frame(&frame) {
                Err(_) => eprintln!("unhandled frame: {:?}", frame),
                Ok(msg) => println!("{:#?}", msg),
            },
        }
    }
    Ok(())
}

fn uart_loop<P: AsRef<OsStr>>(path: &P, baud: u32) -> Result {
    use serialport::prelude::*;

    let port = BufReader::new(serialport::open_with_settings(
        path,
        &SerialPortSettings {
            baud_rate: baud,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(50),
        },
    )?);

    let mut deframer = Deframer::new();

    for b in port.bytes() {
        match b {
            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
            Ok(b) => match deframer.push(b) {
                Err(e) => eprintln!("deframe error {:?}", e),
                Ok(None) => (),
                Ok(Some(frame)) => match Msg::from_frame(&frame) {
                    Err(_) => eprintln!("unhandled frame: {:?}", frame),
                    Ok(msg) => println!("{:#?}", msg),
                },
            },
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn i2c_loop<P: AsRef<Path> + Debug>(path: &P, addr: u16) -> Result {
    use i2c_linux::{I2c, Message as I2cMessage, ReadFlags, WriteFlags};
    use std::thread;
    use ublox::{
        framing::{frame, Frame},
        messages::{cfg, nav, Message},
    };

    let mut dev = I2c::from_path(path)?;
    let mut deframer = Deframer::new();
    let mut scratch = [0x00_u8; 128];

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
        dev.i2c_transfer(&mut msgs)?;
        Ok(u16::from_be_bytes(available).into())
    };

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
        dev.i2c_transfer(&mut msgs)?;
        if let I2cMessage::Write { data: msg_src, .. } = msgs[1] {
            assert_eq!(msg_src.len(), src.len());
        }
        Ok(())
    }

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
        debug!("{:02x?}", &scratch[..len]);
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
        debug!("{:02x?}", &scratch[..len]);
        write(&mut dev, addr, &scratch[..len])?;
    }

    {
        let frm = Frame {
            class: 6,
            id: 1,
            message: vec![nav::Pvt::CLASS, nav::Pvt::ID, 1],
        };
        let en_msg = frm.into_framed_vec();
        debug!("{:x?}", en_msg);
        write(&mut dev, addr, &en_msg)?;
    }

    {
        let frm = Frame {
            class: 6,
            id: 1,
            message: vec![nav::TimeGps::CLASS, nav::TimeGps::ID, 1],
        };
        let en_msg = frm.into_framed_vec();
        debug!("{:x?}", en_msg);
        write(&mut dev, addr, &en_msg)?;
    }

    loop {
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
            warn!(
                "n_avail {} {:#06x} appears to be a glitch, retry",
                n_avail, n_avail
            );
            thread::sleep(Duration::from_millis(50));
        }
        thread::sleep(Duration::from_millis(50));

        if n_avail == 0 {
            thread::sleep(Duration::from_millis(200));
            continue;
        }

        debug!("n_avail {} {:#06x}", n_avail, n_avail);

        let read_len = usize::min(n_avail, scratch.len());
        let read_buf = &mut scratch[..read_len];
        read(&mut dev, addr, read_buf)?;

        for &mut b in read_buf {
            match deframer.push(b) {
                Err(e) => warn!("deframe error {:?}", e),
                Ok(None) => (),
                Ok(Some(frame)) => match Msg::from_frame(&frame) {
                    Err(_) => warn!("unhandled frame: {:?}", frame),
                    Ok(msg) => println!("\n{:?}\n", msg),
                },
            }
        }
    }
}
