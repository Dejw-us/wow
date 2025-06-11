use crate::action::Action;
use crate::attribute::align;
use crate::attribute::geometry::Geometry;
use crate::attribute::style::Style;
use crate::display::Text;
use crate::state::listener::StateListener;
use crate::widget::WidgetEssentials;
use gtk4::glib::WeakRef;
use gtk4::prelude::WidgetExt;
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
  type Widget = Button;

  fn build() -> Self::Widget {
    Button::builder().build()
  }

  fn listener(widget: WeakRef<Self::Widget>) -> StateListener {
    StateListener::Button(widget)
  }
}
