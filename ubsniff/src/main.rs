mod cmdline;
use cmdline::Cmdline;
use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{ErrorKind, Read},
    path::Path,
    time::Duration,
};
use structopt::StructOpt;
use ublox::{framing::Deframer, messages::Msg};

fn main() -> Result<(), Box<dyn Error>> {
    let cmdline = Cmdline::from_args();
    match cmdline {
        Cmdline::File { path } => file_loop(&path),
        Cmdline::I2c { path, addr } => i2c_loop(&path, addr),
        Cmdline::Serial { path, baud } => uart_loop(&path, baud),
    }
}

fn file_loop(path: &Path) -> Result<(), Box<dyn Error>> {
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

fn uart_loop<P: AsRef<OsStr>>(path: &P, baud: u32) -> Result<(), Box<dyn Error>> {
    use serialport::prelude::*;
    let mut port = serialport::open_with_settings(
        path,
        &SerialPortSettings {
            baud_rate: baud,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1),
        },
    )?;

    let mut deframer = Deframer::new();
    let mut buf = [0u8; 256];
    loop {
        match port.read(buf.as_mut()) {
            Err(e) if e.kind() == ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
            Ok(n_read) => {
                for &b in &buf[..n_read] {
                    match deframer.push(b) {
                        Err(e) => eprintln!("deframe error {:?}", e),
                        Ok(None) => (),
                        Ok(Some(frame)) => match Msg::from_frame(&frame) {
                            Err(_) => eprintln!("unhandled frame: {:?}", frame),
                            Ok(msg) => println!("{:#?}", msg),
                        },
                    }
                }
            }
        };
    }
}

fn i2c_loop<P: AsRef<OsStr>>(_path: &P, _addr: u8) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}
