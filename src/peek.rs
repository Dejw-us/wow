pub trait OptionPeek<T> {
  fn if_some(&self, peek: impl FnOnce(&T));
}

impl<T> OptionPeek<T> for Option<T> {
  fn if_some(&self, peek: impl FnOnce(&T)) {
    match self {
      None => {}
      Some(value) => peek(value),
    }
  }
}
