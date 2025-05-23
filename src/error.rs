pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
  Message(String),
  Error,
}

impl Error {
  pub fn print_message(&self) {
    if let Error::Message(message) = self {
      println!("{}", message);
    }
  }
}
