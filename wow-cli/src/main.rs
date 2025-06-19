use crate::cli::Cli;
use clap::Parser;
use std::error::Error;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub mod cli;

const SOCKET: &str = "/tmp/wow.sock";

fn main() -> Result<(), Box<dyn Error>> {
  let command = Cli::parse();

  if let Some(window_name) = command.open() {
    let mut stream = UnixStream::connect(SOCKET)?;
    let msg = format!("open {}", window_name);

    stream.write_all(msg.as_bytes())?;
    stream.flush()?;
  }

  if let Some(window_name) = command.close() {
    let mut stream = UnixStream::connect(SOCKET)?;
    let msg = format!("close {}", window_name);

    stream.write_all(msg.as_bytes())?;
    stream.flush()?;
  }

  Ok(())
}
