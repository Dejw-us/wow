use crate::result::Error;
use crate::state::listener::StateListener;
use crate::value::Value;
use std::cell::{Ref, RefCell};
use std::fmt::Display;
use std::sync::Arc;

pub mod listener;

#[derive(Debug, Clone)]
pub enum StateValue {
  String(String),
  Int(i64),
  Float(f64),
}

impl TryFrom<&serde_yaml::Value> for StateValue {
  type Error = Error;
  fn try_from(value: &serde_yaml::Value) -> Result<Self, Self::Error> {
    match value {
      serde_yaml::Value::String(string) => Ok(Self::String(string.to_string())),
      serde_yaml::Value::Number(num) => {
        if let Some(num) = num.as_i64() {
          Ok(Self::Int(num))
        } else if let Some(num) = num.as_f64() {
          Ok(Self::Float(num))
        } else {
          Err(Error::Message("Failed to read number".into()))
        }
      }
      _ => Err(Error::Message("Failed to create state value".into())),
    }
  }
}

impl Display for StateValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      StateValue::String(string) => string.to_string(),
      StateValue::Int(int) => int.to_string(),
      StateValue::Float(float) => float.to_string(),
    };
    write!(f, "{}", str)
  }
}

pub struct State {
  value: Arc<RefCell<Value>>,
  listeners: RefCell<Vec<StateListener>>,
}

impl State {
  pub fn new(value: Value) -> State {
    State {
      value: Arc::new(RefCell::new(value)),
      listeners: RefCell::new(Vec::new()),
    }
  }

  pub fn subscribe(&self, listener: StateListener) {
    self.listeners.borrow_mut().push(listener);
  }

  pub fn set(&self, value: Value) {
    for listener in self.listeners.borrow().iter() {
      listener.run(&value);
    }
    *self.value.borrow_mut() = value;
  }

  pub fn get(&self) -> Ref<'_, Value> {
    self.value.borrow()
  }
}
