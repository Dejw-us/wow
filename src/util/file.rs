use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file_to_string(filename: &str) -> Result<String, std::io::Error> {
  let file = File::open(filename)?;
  let mut buf_reader = BufReader::new(file);
  let mut buf = String::new();
  buf_reader.read_to_string(&mut buf)?;
  Ok(buf)
}
