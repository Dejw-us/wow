use std::{rc::Rc, time::Duration};

use derive_builder::Builder;
use gtk4::{
  Box as GtkBox, GestureClick, Label,
  glib::{self, ControlFlow, object::Cast, timeout_add_local},
  prelude::{BoxExt, WidgetExt},
};
use serde::Deserialize;

use crate::error;

use super::{
  WidgetRender,
  action::Action,
  label::WidgetLabel,
  margin::Margin,
  orientation::{self, Orientation},
  state::{State, StateValue, StateWidget, WidgetStates},
};

#[derive(Deserialize, Clone)]
pub struct RawContainer {
  label: WidgetLabel,
  #[serde(rename = "state")]
  states: WidgetStates,
  #[serde(rename = "on-click")]
  on_click: Option<String>,
  spacing: i32,
  orientation: Orientation,
  margin: Margin,
  classes: Vec<String>,
}

fn parse_set_command(s: &str) -> Option<(String, String)> {
  if s.starts_with('$') {
    let parts: Vec<&str> = s[1..].split('=').collect();
    if parts.len() == 2 {
      let name = parts[0].to_string();
      let mut val = parts[1].trim().to_string();
      if val.starts_with('"') && val.ends_with('"') {
        val = val[1..val.len() - 1].to_string();
      }
      return Some((name, val));
    }
  }
  None
}

// impl Into<Container> for RawContainer {
//   fn into(self) -> Container {
//     let states_clone = self.states.clone();
//     let mut action: Option<Action> = None;
//     if let Some(raw_action) = self.on_click {
//       if let Some((name, value)) = parse_set_command(&raw_action) {
//         action = Some(Action::new(move || {
//           states_clone.set(&name, StateValue::String(value.to_string()));
//           Ok(())
//         }));
//       }
//     }

//     Container {
//       on_click: action,
//       label: self.label,
//       states: self.states,
//       spacing: self.spacing,
//       orientation: self.orientation,
//       margin: self.margin,
//       classes: self.classes,
//     }
//   }
// }

#[derive(Builder, Clone, Deserialize, Debug)]
#[builder(setter(into))]
pub struct Container {
  label: WidgetLabel,
  #[builder(default, setter(each = "state"))]
  #[serde(rename = "state")]
  states: WidgetStates,
  #[builder(default, setter(custom))]
  #[serde(rename = "on-click")]
  on_click: Option<Action>,
  #[builder(default = "0")]
  spacing: i32,
  #[builder(default = "Orientation::Horizontal")]
  orientation: Orientation,
  #[builder(default)]
  margin: Margin,
  #[builder(default = "vec![]", setter(each = "class"))]
  classes: Vec<String>,
}

impl StateWidget for Container {
  fn states(&self) -> &WidgetStates {
    &self.states
  }
}

impl WidgetRender for Container {
  fn render(&self) -> gtk4::Widget {
    let orientation = &self.orientation;
    let container = GtkBox::new(orientation.into(), self.spacing);
    let box_label = Label::new(None);
    let box_label_clone = box_label.clone();

    if let Some(on_click) = &self.on_click {
      let click = GestureClick::new();
      let on_click = on_click.clone_inner();
      click.connect_pressed(move |_, _, _, _| match on_click() {
        Ok(_) => (),
        Err(err) => err.print_message(),
      });
      box_label.add_controller(click);
    }

    for class in self.classes.iter() {
      container.add_css_class(&class);
    }

    self.margin.set(&container);

    match &self.label {
      WidgetLabel::Exact { label } => box_label.set_label(&label),
      WidgetLabel::State { name } => {
        let state = self.states.get(&name).expect("Failed to find state");

        state.subscribe(move |value| {
          println!("Updated state");
          box_label_clone.set_label(&value.to_string());
        });

        let value = state.get();
        box_label.set_label(&value.to_string());
      }
      WidgetLabel::Empty => (),
    };

    container.append(&box_label);
    container.upcast()
  }
}
