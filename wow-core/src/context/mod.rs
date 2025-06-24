use crate::context::config::get_config_paths;
use crate::functions::{Errors, Files};
use crate::state::State;
use crate::value::Value;
use crate::widget::Widget;
use crate::window::{WindowConfig, WindowConfigStates};
use gtk4::Application;
use log::info;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wow_utils::option::IfSome;

mod config;
mod functions;

pub struct Context {
  states: RefCell<HashMap<String, State>>,
  custom_widgets: HashMap<String, Widget>,
  windows: HashMap<String, (WindowConfig, WindowConfigStates)>,
}

impl Context {
  /// Loads context from ~/.config/wow directory
  pub fn load_from_config() -> crate::Result<Self> {
    let (_, windows_dir, widgets_dir) = get_config_paths()?;

    let windows = fs::read_dir(windows_dir)
      .map_err(Errors::unknown)?
      .filter_map(Result::ok)
      .filter(Files::is_file)
      .filter_map(functions::to_window_entry)
      .collect();
    let widgets = fs::read_dir(widgets_dir)?
      .filter_map(Result::ok)
      .filter(Files::is_file)
      .filter_map(functions::to_widget_entry)
      .collect();
    let context = Context {
      windows,
      custom_widgets: widgets,
      states: RefCell::new(HashMap::new()),
    };

    Ok(context)
  }

  /// Returns custom widget by name from configured widgets in ~/.config/wow/widgets
  pub fn get_custom_widget(&self, name: &str) -> Option<&Widget> {
    self.custom_widgets.get(name)
  }

  /// Opens window by name from configured windows in ~/.config/wow/windows.
  pub fn open_window(context: Rc<Self>, name: &str, app: &Application) {
    context.windows.get(name).if_some(|(window, states)| {
      states.add_states(context.as_ref());
      window.render(app, context.clone(), name);
      info!("Opening window {}", name);
    })
  }

  /// Sets state value in the main wow context.
  /// If state with provided key is not initialized it will create new state.
  pub fn set_state(&self, key: &str, value: Value) {
    info!("Setting state {} = {}", key, value);
    self
      .states
      .borrow_mut()
      .entry(key.to_string())
      .and_modify(|state| state.set(value.clone()))
      .or_insert(State::new(value));
  }

  /// Retrieves state from main wow context using key.
  pub fn get_state(&self, key: &str) -> Option<Ref<'_, State>> {
    info!("Getting state {}", key);
    Ref::filter_map(self.states.borrow(), |map| map.get(key)).ok()
  }
}
