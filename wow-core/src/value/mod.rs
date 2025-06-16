use crate::object::Object;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  None,
  Object(Object),
  I64(i64),
  F64(f64),
  Bool(bool),
  String(String),
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Value::None => String::new(),
      Value::I64(int) => int.to_string(),
      Value::F64(float) => float.to_string(),
      Value::Bool(bool) => bool.to_string(),
      Value::String(string) => string.to_string(),
      Value::Object(object) => format!("{:?}", object),
    };
    write!(f, "{}", str)
  }
}

impl From<String> for Value {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}

impl From<i64> for Value {
  fn from(value: i64) -> Self {
    Self::I64(value)
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Self::F64(value)
  }
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::Bool(value)
  }
}

impl From<&serde_yaml::Value> for Value {
  fn from(value: &serde_yaml::Value) -> Self {
    match value {
      serde_yaml::Value::String(string) => Self::String(string.to_string()),
      serde_yaml::Value::Number(num) => {
        if let Some(num) = num.as_i64() {
          Self::I64(num)
        } else if let Some(num) = num.as_f64() {
          Self::F64(num)
        } else {
          Self::None
        }
      }
      serde_yaml::Value::Bool(b) => Self::Bool(*b),
      _ => Self::None,
    }
  }
}
