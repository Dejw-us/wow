#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Value {
  I32(i32),
  F32(f32),
  Bool(bool),
  String(String),
}

impl Into<String> for Value {
  fn into(self) -> String {
    match self {
      Value::I32(int) => int.to_string(),
      Value::F32(float) => float.to_string(),
      Value::Bool(bool) => bool.to_string(),
      Value::String(string) => string,
    }
  }
}

impl From<String> for Value {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}

impl From<i32> for Value {
  fn from(value: i32) -> Self {
    Self::I32(value)
  }
}

impl From<f32> for Value {
  fn from(value: f32) -> Self {
    Self::F32(value)
  }
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::Bool(value)
  }
}
