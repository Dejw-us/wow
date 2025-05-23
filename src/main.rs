use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::{env::set_var, time::Duration};

use derive_builder::Builder;
use error::Error;
use gtk4::glib::ControlFlow;
use gtk4::{
  Application, ApplicationWindow, Box as GtkBox, Label, Orientation,
  gio::prelude::{ApplicationExt, ApplicationExtManual},
  glib::timeout_add_local,
  prelude::{GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use widget::Widget;
use widget::container::ContainerBuilder;
use widget::margin::MarginBuilder;
use widget::{
  WidgetRender,
  container::Container,
  label::WidgetLabel,
  state::{State, StateValue},
};
use window::{Window, WindowBuilder};

pub mod error;
pub mod widget;
pub mod window;

fn main() {
  gtk4::init().expect("Failed to init gtk");

  let child = ContainerBuilder::default()
    .label(WidgetLabel::Exact("Hello".to_string()))
    .build()
    .expect("Failed to build container");

  let window = WindowBuilder::default()
    .id("me.test".to_string())
    .child(Some(Widget::Container(child)))
    .build()
    .expect("Failed to build window");

  window.open(true);
}
