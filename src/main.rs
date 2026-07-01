use std::io::Write;

use anyhow::Context;
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    /// The serial port to use
    #[clap(short, long)]
    port: String,

    /// The baud rate to use
    #[clap(short, long)]
    baud_rate: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut port = serialport::new(args.port, args.baud_rate)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .context("Serial Port open fail")?;

    loop {
        let mut bytes = vec![0; 1024];

        match port.read(&mut bytes) {
            Ok(len) => {
                std::io::stdout()
                    .write_all(&bytes[..len])
                    .context("Failed to write to stdout")?;
                std::io::stdout()
                    .flush()
                    .context("Failed to flush stdout")?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(e) => {
                println!("Serial Port read fail: {}", e);
            }
        }

        // print!("{}", msg);
    }
}
