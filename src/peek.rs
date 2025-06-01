pub trait PeekOption<T> {
  fn peek(&self, peek: impl Fn(&T));
}

impl<T> PeekOption<T> for Option<T> {
  fn peek(&self, peek: impl Fn(&T)) {
    match self {
      None => {}
      Some(value) => peek(value),
    }
  }
}
