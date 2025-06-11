use crate::context::Context;
use crate::widget::ApplyWidget;
use gtk4::prelude::WidgetExt;
use serde::Deserialize;
use std::rc::Rc;
use wow_utils::option::IfSome;

#[derive(Debug, Deserialize, Clone)]
pub struct Geometry {
  #[serde(rename = "min-width")]
  min_width: Option<i32>,
  #[serde(rename = "min-height")]
  min_height: Option<i32>,
}

impl ApplyWidget for Geometry {
  fn apply(&self, widget: &impl WidgetExt, context: Rc<Context>) {
    self.min_width.if_some(|w| widget.set_width_request(*w));
    self.min_height.if_some(|h| widget.set_height_request(*h));
  }
}
