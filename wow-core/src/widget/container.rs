use crate::attribute::align::Align;
use crate::attribute::geometry::Geometry;
use crate::attribute::orientation::Orientation;
use crate::attribute::style::Style;
use crate::state::listener::StateListener;
use crate::widget::{Widget, WidgetEssentials};
use gtk4::glib::WeakRef;
use serde::Deserialize;
use std::fmt::Debug;
use wow_derive::RenderWidget;

#[derive(Debug, RenderWidget, Deserialize)]
pub struct ContainerConfig {
  childs: Vec<Widget>,
  orientation: Option<Orientation>,
  geometry: Option<Geometry>,
  style: Option<Style>,
  align: Option<Align>,
}

impl WidgetEssentials for ContainerConfig {
  type Widget = gtk4::Box;

  fn build() -> Self::Widget {
    gtk4::Box::builder().build()
  }

  fn listener(widget: WeakRef<Self::Widget>) -> StateListener {
    StateListener::None
  }
}
