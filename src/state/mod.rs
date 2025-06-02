use crate::state::listener::StateListener;
use std::cell::{Ref, RefCell};
use std::fmt::Display;
use std::sync::Arc;

pub mod listener;

#[derive(Debug, Clone)]
pub enum StateValue {
  String(String),
  Int(i32),
  Float(f32),
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
  value: Arc<RefCell<StateValue>>,
  listeners: RefCell<Vec<StateListener>>,
}

impl State {
  pub fn new(value: StateValue) -> State {
    State {
      value: Arc::new(RefCell::new(value)),
      listeners: RefCell::new(Vec::new()),
    }
  }

  pub fn subscribe(&self, listener: StateListener) {
    self.listeners.borrow_mut().push(listener);
  }

  pub fn set(&self, value: StateValue) {
    for listener in self.listeners.borrow().iter() {
      listener.run(&value);
    }
    *self.value.borrow_mut() = value;
  }

  pub fn get(&self) -> Ref<'_, StateValue> {
    self.value.borrow()
  }
}
