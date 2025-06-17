use crate::attribute::align::Align;
use crate::attribute::geometry::Geometry;
use crate::attribute::style::Style;
use crate::display::Text;
use crate::widget::WidgetEssentials;
use gtk4::prelude::Cast;
use gtk4::Label;
use serde::Deserialize;
use wow_derive::RenderWidget;

#[derive(Deserialize, Debug, RenderWidget)]
pub struct LabelConfig {
  label: Text,
  style: Option<Style>,
  geometry: Option<Geometry>,
  align: Option<Align>,
}

impl WidgetEssentials for LabelConfig {
  fn build() -> gtk4::Widget {
    Label::builder().build().upcast()
  }
}
