use crate::config::style::Style;
use crate::config::widget::{Render, Widget};
use crate::config::window::anchor::WindowAnchor;
use crate::config::window::layer::WindowLayer;
use crate::config::ApplyWidget;
use crate::context::Context;
use crate::peek::OptionPeek;
use crate::state::StateValue;
use gtk4::prelude::{Cast, GtkWindowExt, WidgetExt};
use gtk4::Application;
use gtk4_layer_shell::LayerShell;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub mod anchor;
pub mod layer;

#[derive(Deserialize, Debug)]
pub struct WindowConfig {
  child: Widget,
  anchor: Vec<WindowAnchor>,
  layer: Option<WindowLayer>,
  style: Option<Style>,
}

pub struct WindowConfigStates {
  states: HashMap<String, StateValue>,
}

impl WindowConfigStates {
  pub fn add_states(&self, context: &Context) {
    self
      .states
      .iter()
      .for_each(|(name, state)| context.set_state_value(name, state.clone()));
  }
}

impl<'de> Deserialize<'de> for WindowConfigStates {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let data: HashMap<String, StateValue> = HashMap::<String, Value>::deserialize(deserializer)?
      .iter()
      .filter_map(|(name, value)| match StateValue::try_from(value) {
        Ok(value) => Some((name[1..].to_string(), value)),
        Err(_) => None,
      })
      .collect();
    Ok(Self { states: data })
  }
}

impl WindowConfig {
  pub fn render(&self, app: &Application, context: Rc<Context>) {
    let window = gtk4::Window::builder()
      .application(app)
      .child(&self.child.render(context))
      .build();

    self.style.if_some(|style| style.apply(&window));

    self.layer.if_some(|layer| {
      window.init_layer_shell();
      window.set_layer(layer.clone().into());
    });

    for anchor in &self.anchor {
      window.set_anchor(anchor.clone().into(), true);
    }

    window.auto_exclusive_zone_enable();

    window.present();
  }
}
