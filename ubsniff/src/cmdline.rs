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
    #[cfg(target_os = "linux")]
    I2c {
        /// Path to I2C dev.
        #[structopt(name = "PATH")]
        path: PathBuf,
        /// I2C bus address.
        #[structopt(name = "ADDR", default_value = "0x42", parse(try_from_str = u16::from_hex_dec_bin))]
        addr: u16,
        /// TX data ready pin.
        #[structopt(name = "PIN", short = "p", long = "pin")]
        tx_ready_pin: Option<u64>,
    },
}

trait FromHexDecBin: Sized {
    type Error;
    fn from_hex_dec_bin(s: &str) -> Result<Self, Self::Error>;
}

macro_rules! impl_from_hex_dec_bin {
    ($T:tt, $E:ty) => {
        impl FromHexDecBin for $T {
            type Error = $E;
            fn from_hex_dec_bin(s: &str) -> Result<$T, Self::Error> {
                if s.len() > 2 {
                    match s.split_at(2) {
                        ("0x", rest) => $T::from_str_radix(rest, 16),
                        ("0b", rest) => $T::from_str_radix(rest, 2),
                        _ => $T::from_str_radix(s, 10),
                    }
                } else {
                    $T::from_str_radix(s, 10)
                }
            }
        }
    };
}

impl_from_hex_dec_bin!(u16, ::std::num::ParseIntError);
