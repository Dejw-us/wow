use crate::context::Context;
use crate::widget::ApplyWidget;
use gtk4::prelude::WidgetExt;
use gtk4::Widget;
use log::debug;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
pub struct Expand {
  vertical: Option<bool>,
  horizontal: Option<bool>,
}

impl ApplyWidget for Expand {
  fn apply(&self, widget: &Widget, _context: Rc<Context>) {
    if let Some(v) = self.vertical {
      debug!("setting vertical expand to {}", v);
      widget.set_vexpand(v);
    }
    if let Some(h) = self.horizontal {
      debug!("setting horizontal expand to {}", h);
      widget.set_hexpand(h);
    }
  }
}
