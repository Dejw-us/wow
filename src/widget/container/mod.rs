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
  state::{State, StateValue, WidgetStates},
};

#[derive(Builder, Clone, Deserialize)]
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

impl ContainerBuilder {
  pub fn on_click(&mut self, on_click: Action) -> &mut Self {
    self.on_click = Some(Some(on_click));
    self
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
      WidgetLabel::Exact(label) => box_label.set_label(&label),
      WidgetLabel::State(name) => {
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
