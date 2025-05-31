use std::cell::{Ref, RefCell};
use std::fmt::Display;
use std::rc::Rc;

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
  value: Rc<RefCell<StateValue>>,
  listeners: Vec<Box<dyn Fn(&StateValue) + 'static>>,
}

impl State {
  pub fn new(value: StateValue) -> State {
    State {
      value: Rc::new(RefCell::new(value)),
      listeners: Vec::new(),
    }
  }

  pub fn subscribe_set(&mut self, listener: impl Fn(&StateValue) + 'static) {
    self.listeners.push(Box::new(listener));
  }

  pub fn set(&mut self, value: StateValue) {
    for listener in self.listeners.iter() {
      listener(&value);
    }
    *self.value.borrow_mut() = value;
  }

  pub fn get(&self) -> Ref<'_, StateValue> {
    self.value.borrow()
  }
}
