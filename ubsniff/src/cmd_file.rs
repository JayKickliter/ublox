use crate::error::Result;
use std::{fs::File, io::Read, path::Path};
use ublox::{framing::Deframer, messages::Msg};

pub fn file_loop(path: &Path) -> Result {
    let file = File::open(path)?;

    let mut deframer = Deframer::new();
    for b in file.bytes() {
        match deframer.push(b?) {
            None => (),
            Some(frame) => match Msg::from_frame(&frame) {
                Err(_) => eprintln!("unhandled frame: {:?}", frame),
                Ok(msg) => println!("{:#?}", msg),
            },
        }
    }
    Ok(())
}
