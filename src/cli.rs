use clap::{Parser, Subcommand};

/// a serialport terminal
#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// list all available serial ports
    List,

    /// connect to a serial terminal
    Connect {
        /// The serial port to use
        port: String,

        /// The baud rate to use
        #[clap(short, long, default_value_t = 115200)]
        baud_rate: u32,
    },
}
