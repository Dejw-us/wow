use crate::state::{State, StateValue};
use std::cell::Ref;
use std::collections::HashMap;

pub struct Context {
  states: HashMap<String, State>,
}

impl Context {
  pub fn with_states(states: HashMap<String, State>) -> Self {
    Self { states }
  }

  pub fn new() -> Self {
    Self {
      states: HashMap::new(),
    }
  }

  pub fn set_state(&mut self, key: String, state: State) {
    self.states.insert(key, state);
  }

  pub fn get_state(&self, key: &str) -> Option<&State> {
    self.states.get(key)
  }

  pub fn get_state_value(&self, key: &str) -> Option<Ref<'_, StateValue>> {
    self.states.get(key).map(|s| s.get())
  }
}
