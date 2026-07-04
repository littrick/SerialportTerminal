mod cli;
mod terminal;

use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use cli::*;
use serialport::SerialPortInfo;
use terminal::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

pub async fn run_terminal() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::List => list()?,
        Command::Connect { port, baud_rate } => connect(port, baud_rate).await?,
    }

    Ok(())
}

fn list() -> anyhow::Result<()> {
    let ports = serialport::available_ports().context("Fail to get available serial ports")?;

    ports
        .iter()
        .for_each(|port| println!("{}", serialport2str(port)));

    Ok(())
}

fn serialport2str(info: &SerialPortInfo) -> String {
    let type_str = match &info.port_type {
        serialport::SerialPortType::UsbPort(i) => {
            let mut str = String::default();
            if let Some(p) = &i.product {
                str.push_str(&format!("{p}"));
            }

            str.push_str(&format!(", {:X}:{:X}", i.vid, i.pid));

            if let Some(sn) = &i.serial_number {
                str.push_str(&format!(", sn({sn})"));
            }

            if let Some(m) = &i.manufacturer {
                str.push_str(&format!(", manufacturer({m})"));
            }
            str
        }
        serialport::SerialPortType::PciPort => "PCI port".to_string(),
        serialport::SerialPortType::BluetoothPort => "Bluetooth port".to_string(),
        serialport::SerialPortType::Unknown => "Unknown".to_string(),
    };

    format!("[{}]: {}", info.port_name, type_str)
}

async fn connect(name: String, baud_rate: u32) -> anyhow::Result<()> {
    let port = serialport::new(name, baud_rate).open()?;
    let one_port = Arc::new(Mutex::new(port));
    let terminal = Terminal::new();
    let (mut reader, mut writer) = terminal.split();
    writer
        .write_all(b"Connected to serial port. Press `CTRL+A CTRL+A` to exit.\n")
        .await?;

    let port = one_port.clone();
    let read_task = async move {
        let mut buf = vec![0; 1024];
        loop {
            match port.lock().await.read(buf.as_mut_slice()) {
                Ok(len) => {
                    writer.write(&buf[..len]).await.unwrap();
                    writer.flush().await.unwrap();
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
            tokio::task::yield_now().await;
        }
    };

    let port = one_port.clone();
    let write_task = async move {
        let mut buf = vec![0; 1024];
        let mut ctl_a_pressed = false;
        loop {
            let n = reader.read(buf.as_mut_slice()).await.unwrap();
            if n > 0 {
                if n == 1 {
                    if ctl_a_pressed && buf[0] == 0x01 {
                        println!("Ctrl+A Ctrl+A detected. Exiting...");
                        break;
                    }
                    ctl_a_pressed = buf[0] == 0x01; // Ctrl+A
                }

                let mut port = port.lock().await;
                port.write_all(&buf[..n]).unwrap();
                port.flush().unwrap();
            }
        }
    };

    tokio::select! {
        _ = read_task => {},
        _ = write_task => {}
    }

    eprintln!("Terminal session ended.");
    Ok(())
}
