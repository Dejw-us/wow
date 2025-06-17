use crate::action::Action;
use crate::attribute::align;
use crate::attribute::geometry::Geometry;
use crate::attribute::style::Style;
use crate::display::Text;
use crate::widget::WidgetEssentials;
use gtk4::prelude::{Cast, WidgetExt};
use gtk4::Button;
use serde::Deserialize;
use wow_derive::RenderWidget;

#[derive(Deserialize, Debug, RenderWidget)]
pub struct ButtonConfig {
  label: Text,
  #[serde(rename = "on-click")]
  on_click: Option<Action>,
  style: Option<Style>,
  geometry: Option<Geometry>,
  align: Option<align::Align>,
}

impl WidgetEssentials for ButtonConfig {
  fn build() -> gtk4::Widget {
    Button::builder().build().upcast()
  }
}
