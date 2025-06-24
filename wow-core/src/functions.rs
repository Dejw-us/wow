use std::fs::DirEntry;

pub struct Chars;

pub struct Mappings;

impl Mappings {
  pub fn into<T: Into<E>, E>(into: T) -> E {
    into.into()
  }
}

pub struct Errors;

impl Errors {
  pub fn unknown(err: impl std::error::Error + 'static + Send + Sync) -> crate::Error {
    crate::Error::Unknown(Box::new(err))
  }
}

impl Chars {
  pub fn is_whitespace(char: &char) -> bool {
    char.is_whitespace()
  }

  pub fn not_whitespace(char: &char) -> bool {
    !Self::is_whitespace(char)
  }
}

pub struct Files;

impl Files {
  pub fn is_file(entry: &DirEntry) -> bool {
    entry.path().is_file()
  }
}

pub fn ok<T, E>(result: Result<T, E>) -> Option<T> {
  result.ok()
}
