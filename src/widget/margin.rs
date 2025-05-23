use derive_builder::Builder;
use gtk4::prelude::WidgetExt;
use serde::Deserialize;

#[derive(Builder, Default, Clone, Deserialize)]
#[builder(default)]
#[serde(default)]
pub struct Margin {
  top: i32,
  bottom: i32,
  left: i32,
  right: i32,
}

impl Margin {
  pub fn set(&self, widget: &impl WidgetExt) {
    widget.set_margin_bottom(self.bottom);
    widget.set_margin_top(self.top);
    widget.set_margin_end(self.right);
    widget.set_margin_start(self.left);
  }
}
