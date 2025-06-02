use crate::state::{State, StateValue};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;

pub struct Context {
  states: RefCell<HashMap<String, State>>,
}

impl Context {
  pub fn with_states(states: HashMap<String, State>) -> Self {
    Self {
      states: RefCell::new(states),
    }
  }

  pub fn set_state_value(&self, key: &str, value: StateValue) {
    let mut states = self.states.borrow_mut();
    match states.get(key) {
      None => {
        states.insert(key.to_string(), State::new(value));
      }
      Some(state) => {
        state.set(value);
      }
    }
  }

  pub fn get_state(&self, key: &str) -> Option<Ref<'_, State>> {
    Ref::filter_map(self.states.borrow(), |map| map.get(key)).ok()
  }
}
