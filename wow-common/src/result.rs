use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Message(String),
  Unknown,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::Message(msg) => write!(f, "{}", msg),
      Error::Unknown => write!(f, "Unknown error"),
    }
  }
}
