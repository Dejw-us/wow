use dirs::config_dir;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file_to_string(filename: &str) -> Result<String, std::io::Error> {
  let config_dir = config_dir()
    .expect("Failed to get config directory")
    .to_str()
    .expect("Failed to get config directory")
    .to_string();
  let path = format!("{}/wow/{}", config_dir, filename);
  let file = File::open(path)?;
  let mut buf_reader = BufReader::new(file);
  let mut buf = String::new();
  buf_reader.read_to_string(&mut buf)?;
  Ok(buf)
}
