use crate::attribute::align::Align;
use crate::attribute::expand::Expand;
use crate::attribute::geometry::Geometry;
use crate::attribute::orientation::Orientation;
use crate::attribute::spacing::Spacing;
use crate::attribute::style::Style;
use crate::widget::{Widget, WidgetEssentials};
use gtk4::prelude::{Cast, WidgetExt};
use serde::Deserialize;
use std::fmt::Debug;
use wow_derive::RenderWidget;

#[derive(Debug, RenderWidget, Deserialize)]
pub struct ContainerConfig {
  childs: Option<Vec<Widget>>,
  orientation: Option<Orientation>,
  geometry: Option<Geometry>,
  style: Option<Style>,
  align: Option<Align>,
  spacing: Option<Spacing>,
  expand: Option<Expand>,
}

impl WidgetEssentials for ContainerConfig {
  fn build() -> gtk4::Widget {
    gtk4::Box::builder().build().upcast()
  }
}
