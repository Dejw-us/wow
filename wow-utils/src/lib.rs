use dirs::config_dir;
use std::fs::File;
use std::io::{BufReader, Read};

pub mod map;
pub mod option;

pub fn read_file_to_string(filename: &str) -> Result<String, std::io::Error> {
  let config = config_dir()
    .expect("Failed to get config directory")
    .to_str()
    .expect("Failed to get config directory")
    .to_string();
  let path = format!("{}/wow/{}", config, filename);
  let file = File::open(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut buf = String::new();
  buf_reader.read_to_string(&mut buf)?;
  Ok(buf)
}
