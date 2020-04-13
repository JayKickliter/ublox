use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cmdline {
    /// Print u-blox messages from a file.
    File {
        /// Path to captured messages.
        #[structopt(name = "PATH")]
        path: PathBuf,
    },
    /// Print u-blox messages from a serial port.
    Serial {
        /// Path to TTY
        #[structopt(name = "PATH")]
        path: PathBuf,
        /// Baud rate.
        #[structopt(default_value = "9600")]
        baud: u32,
    },
    I2c {
        /// Path to I2C dev.
        #[structopt(name = "PATH")]
        path: PathBuf,
        /// I2C bus address.
        #[structopt(name = "ADDR")]
        addr: u8,
    },
}
