use crate::context::Context;
use crate::widget::ApplyWidget;
use gtk4::prelude::{Cast, OrientableExt, WidgetExt};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
  Vertical,
  Horizontal,
}

impl ApplyWidget for Orientation {
  fn apply(&self, widget: &impl WidgetExt, context: Rc<Context>) {
    let widget = widget.upcast_ref();
    if let Some(container) = widget.downcast_ref::<gtk4::Box>() {
      println!("Setting orientation to {:?}", self);
      container.set_orientation(self.into());
    }
  }
}

impl Into<gtk4::Orientation> for &Orientation {
  fn into(self) -> gtk4::Orientation {
    match self {
      Orientation::Vertical => gtk4::Orientation::Vertical,
      Orientation::Horizontal => gtk4::Orientation::Horizontal,
    }
  }
}
