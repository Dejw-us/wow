use crate::config::window::WindowConfig;
use crate::peek::OptionPeek;
use crate::state::{State, StateValue};
use gtk4::Application;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Context {
  states: RefCell<HashMap<String, State>>,
  windows: HashMap<String, WindowConfig>,
}

impl Context {
  pub fn new(states: HashMap<String, State>, windows: HashMap<String, WindowConfig>) -> Self {
    Self {
      states: RefCell::new(states),
      windows,
    }
  }

  pub fn open_window(context: Rc<Self>, name: &str, app: &Application) {
    context
      .windows
      .get(name)
      .if_some(|w| w.render(app, context.clone()))
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
