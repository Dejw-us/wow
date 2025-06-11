use crate::functions;
use crate::state::{State, StateValue};
use crate::window::{WindowConfig, WindowConfigStates};
use gtk4::Application;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wow_utils::option::IfSome;

pub struct Context {
  states: RefCell<HashMap<String, State>>,
  windows: HashMap<String, (WindowConfig, WindowConfigStates)>,
}

impl Context {
  /// Loads context from .config/wow directory
  pub fn load() -> std::io::Result<Self> {
    let config_dir = dirs::config_dir()
      .expect("Failed to get config directory")
      .to_str()
      .expect("Failed to convert config directory path to str")
      .to_string();
    println!("Config directory: {}", config_dir);
    let windows_dir = format!("{}/wow/{}", config_dir, "windows");
    let windows = fs::read_dir(windows_dir)?
      .filter_map(Result::ok)
      .filter(functions::is_file)
      .filter_map(functions::to_window_entry)
      .collect();

    let context = Context {
      windows,
      states: RefCell::new(HashMap::new()),
    };

    Ok(context)
  }

  pub fn open_window(context: Rc<Self>, name: &str, app: &Application) {
    context.windows.get(name).if_some(|w| {
      let window = &w.0;
      let states = &w.1;

      states.add_states(context.as_ref());
      window.render(app, context.clone());
    })
  }

  pub fn set_state_value(&self, key: &str, value: StateValue) {
    println!("Adding state {} = {}", key, value);
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
