use crate::config::window::WindowConfig;
use crate::peek::OptionPeek;
use crate::state::{State, StateValue};
use gtk4::Application;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

pub struct Context {
  states: RefCell<HashMap<String, State>>,
  windows: HashMap<String, WindowConfig>,
}

impl Context {
  pub fn load() -> std::io::Result<Self> {
    let config_dir = "~/.config/wow";
    let windows_dir = format!("{}/{}", config_dir, "windows");
    let windows = fs::read_dir(windows_dir)?
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_file())
      .filter_map(|e| fs::read_to_string(&e.path()).ok().map(|s| (e.file_name(), s))
      .filter_map(|e| {

        match data {
          Ok(data) => match serde_yaml::from_str::<WindowConfig>(&data) {
            Ok(window_config) => {
              if let Ok(name) = e.file_name().into_string() {
                Some((name, window_config))
              } else {
                None
              }
            }
            Err(_) => {
              println!(
                "Failed to create window config from {}. check for syntax error",
                e.file_name()
                  .into_string()
                  .unwrap_or("Invalid filename".to_string())
              );
              None
            }
          },
          Err(_) => {
            println!(
              "Failed to load window from file {}",
              e.file_name()
                .into_string()
                .unwrap_or("Invalid filename".to_string())
            );
            None
          }
        }
      })
      .collect();

    let context = Context {
      windows,
      states: RefCell::new(HashMap::new()),
    };

    Ok(context)
  }

  pub fn open_window(context: Rc<Self>, name: &str, app: &Application) {
    context
      .windows
      .get(name)
      .if_some(|w| w.render(app, context.clone()))
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
