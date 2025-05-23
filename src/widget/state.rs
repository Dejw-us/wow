use std::{cell::RefCell, collections::HashMap, env::var_os, fmt::Debug, rc::Rc};

use getset::Getters;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub enum StateValue {
  #[default]
  None,
  String(String),
  Int(i64),
  Float(f64),
}

impl From<serde_yaml::Value> for StateValue {
  fn from(value: serde_yaml::Value) -> Self {
    if let Some(value) = value.as_str() {
      StateValue::String(value.to_string())
    } else if let Some(int) = value.as_i64() {
      StateValue::Int(int)
    } else if let Some(float) = value.as_f64() {
      StateValue::Float(float)
    } else {
      StateValue::None
    }
  }
}

impl ToString for StateValue {
  fn to_string(&self) -> String {
    match self {
      StateValue::String(string) => string.to_string(),
      StateValue::Int(int) => int.to_string(),
      StateValue::Float(float) => float.to_string(),
      StateValue::None => String::new(),
    }
  }
}

#[derive(Clone, Default, Debug)]
pub struct WidgetStates(HashMap<String, State>);

impl<'de> Deserialize<'de> for WidgetStates {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let map: HashMap<String, serde_yaml::Value> = HashMap::deserialize(deserializer)?;
    let mut states = HashMap::new();

    for (key, value) in map {
      let value: StateValue = value.into();

      states.insert(key, State::new(value));
    }

    Ok(WidgetStates::new(states))
  }
}

impl WidgetStates {
  pub fn new(states: HashMap<String, State>) -> Self {
    Self(states)
  }

  pub fn get(&self, name: &str) -> Option<&State> {
    self.0.get(name)
  }
}

#[derive(Clone, Default)]
pub struct State {
  value: Rc<RefCell<StateValue>>,
  listeners: Rc<RefCell<Vec<Box<dyn Fn(&StateValue)>>>>,
}

impl Debug for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("State").field("value", &self.value).finish()
  }
}

impl State {
  pub fn new(value: StateValue) -> Self {
    State {
      value: Rc::new(RefCell::new(value)),
      listeners: Rc::new(RefCell::new(vec![])),
    }
  }

  pub fn subscribe(&self, listener: impl Fn(&StateValue) + 'static) {
    self.listeners.borrow_mut().push(Box::new(listener));
  }

  pub fn set(&self, value: StateValue) {
    *self.value.borrow_mut() = value;
    let value_ref = self.value.borrow();
    for listener in self.listeners.borrow().iter() {
      listener(&value_ref);
    }
  }

  pub fn get(&self) -> StateValue {
    self.value.borrow().clone()
  }
}
