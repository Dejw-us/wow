use crate::functions;
use crate::state::State;
use crate::value::Value;
use crate::widget::Widget;
use crate::window::{WindowConfig, WindowConfigStates};
use gtk4::Application;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wow_utils::option::IfSome;

pub struct Context {
  states: RefCell<HashMap<String, State>>,
  custom_widgets: HashMap<String, Widget>,
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
    let widgets_dir = format!("{}/wow/{}", config_dir, "widgets");
    let windows = fs::read_dir(windows_dir)?
      .filter_map(Result::ok)
      .filter(functions::is_file)
      .filter_map(functions::to_window_entry)
      .collect();
    let widgets = fs::read_dir(widgets_dir)?
      .filter_map(Result::ok)
      .filter(functions::is_file)
      .filter_map(|f| {
        let file_name = f
          .file_name()
          .to_string_lossy()
          .strip_suffix(".yml")
          .ok_or("Failed to strip .yml suffix")
          .unwrap()
          .to_string();
        let content = fs::read_to_string(f.path()).unwrap();
        let widget = serde_yaml::from_str::<Widget>(&content).unwrap();
        println!("Adding widget {:?}", &file_name);
        Some((file_name, widget))
      })
      .collect();
    let context = Context {
      windows,
      custom_widgets: widgets,
      states: RefCell::new(HashMap::new()),
    };

    Ok(context)
  }

  pub fn get_custom_widget(&self, name: &str) -> Option<&Widget> {
    self.custom_widgets.get(name)
  }

  pub fn open_window(context: Rc<Self>, name: &str, app: &Application) {
    context.windows.get(name).if_some(|w| {
      println!("Opening window {}", name);
      let window = &w.0;
      let states = &w.1;

      states.add_states(context.as_ref());
      window.render(app, context.clone(), name);
    })
  }

  pub fn set_state_value(&self, key: &str, value: Value) {
    println!("Adding state {} = {:?}", key, value);
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
