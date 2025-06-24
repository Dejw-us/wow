use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Config directory not found")]
  ConfigDirNotFound,
  #[error("Failed to convert buffer to UTF-8 string")]
  Utf8Error,
  #[error("{0}")]
  Message(String),
  #[error("Error caused by: {0}")]
  Unknown(Box<dyn std::error::Error + Send + Sync>),
  #[error("Failed to create config directory: {0}")]
  FailedToCreateConfigDir(String),
}

impl From<&str> for Error {
  fn from(value: &str) -> Self {
    Self::Message(value.into())
  }
}
