pub trait IfSome<T> {
  fn if_some(&self, peek: impl FnOnce(&T));
}

impl<T> IfSome<T> for Option<T> {
  fn if_some(&self, peek: impl FnOnce(&T)) {
    match self {
      None => {}
      Some(value) => peek(value),
    }
  }
}
