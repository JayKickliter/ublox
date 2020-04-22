mod cmd_file;
#[cfg(target_os = "linux")]
mod cmd_i2c;
mod cmd_uart;
mod cmdline;
mod error;
use cmdline::Cmdline;
use structopt::StructOpt;

fn main() {
    let cmdline = Cmdline::from_args();
    env_logger::init();
    let res = match cmdline {
        Cmdline::File { path } => cmd_file::file_loop(&path),
        #[cfg(target_os = "linux")]
        Cmdline::I2c {
            path,
            addr,
            tx_ready_pin,
        } => cmd_i2c::i2c_loop(&path, addr, tx_ready_pin),
        Cmdline::Serial { path, baud } => cmd_uart::uart_loop(&path, baud),
    };
    if let Err(e) = res {
        eprintln!("error: {}", e);
        ::std::process::exit(1);
    }
}
