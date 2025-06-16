use crate::attribute::style::Style;
use crate::context::Context;
use crate::widget::Widget;
use crate::widget::{ApplyWidget, RenderWidget};
use crate::window::anchor::WindowAnchor;
use crate::window::layer::WindowLayer;
use gtk4::prelude::{GskRendererExt, GtkWindowExt, SettingsExt};
use gtk4::Application;
use gtk4_layer_shell::LayerShell;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::collections::HashMap;
use std::rc::Rc;
use wow_utils::option::IfSome;

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
  states: HashMap<String, crate::value::Value>,
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
    let data: HashMap<String, crate::value::Value> =
      HashMap::<String, Value>::deserialize(deserializer)?
        .iter()
        .filter_map(|(name, value)| match crate::value::Value::from(value) {
          crate::value::Value::None => None,
          value => Some((name.to_string(), value)),
        })
        .collect();
    Ok(Self { states: data })
  }
}

impl WindowConfig {
  pub fn render(&self, app: &Application, context: Rc<Context>) {
    let window = gtk4::Window::builder()
      .application(app)
      .child(&self.child.render(context.clone()))
      .build();

    self
      .style
      .if_some(|style| style.apply(&window, context.clone()));

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
