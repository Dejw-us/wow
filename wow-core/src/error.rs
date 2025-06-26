use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! bail {
  ($e:expr) => {
    return Err($crate::err!($e));
  };
  ($fmt:literal $(, $arg:expr)+ $(,)?) => {
    return Err($crate::err!($fmt $(, $arg)*));
  };
  ($fmt:literal) => {
    return Err($crate::err!($fmt));
  }
}

#[macro_export]
macro_rules! err {
  ($fmt:literal) => {
    $crate::error::Error::Message($fmt.to_string())
  };
  ($e:expr) => {
    $crate::error::Error::Unknown($e)
  };
  ($fmt:literal $(, $arg:expr) + $(,)?) => {
    $crate::error::Error::Message(format!($fmt $(, $arg)*))
  };
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("Env var {0} is not set")]
  EnvVarNotFound(String),
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

impl From<String> for Error {
  fn from(value: String) -> Self {
    Self::from(value.as_str())
  }
}

impl From<&str> for Error {
  fn from(value: &str) -> Self {
    Self::Message(value.into())
  }
}
