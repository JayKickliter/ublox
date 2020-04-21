use crate::error::Result;
use std::{
    ffi::OsStr,
    io::{BufReader, ErrorKind, Read},
    time::Duration,
};
use ublox::{framing::Deframer, messages::Msg};

pub fn uart_loop<P: AsRef<OsStr>>(path: &P, baud: u32) -> Result {
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
                None => (),
                Some(frame) => match Msg::from_frame(&frame) {
                    Err(_) => eprintln!("unhandled frame: {:?}", frame),
                    Ok(msg) => println!("{:#?}", msg),
                },
            },
        }
    }
    Ok(())
}
