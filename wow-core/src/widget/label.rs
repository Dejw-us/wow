use crate::attribute::align::Align;
use crate::attribute::geometry::Geometry;
use crate::attribute::style::Style;
use crate::display::Text;
use crate::state::listener::StateListener;
use crate::widget::WidgetEssentials;
use gtk4::glib::WeakRef;
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
  type Widget = Label;

  fn build() -> Self::Widget {
    Label::builder().build()
  }

  fn listener(widget: WeakRef<Self::Widget>) -> StateListener {
    StateListener::Label(widget)
  }
}
