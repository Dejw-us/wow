use crate::context::Context;
use gtk4::prelude::WidgetExt;
use std::rc::Rc;

pub trait RenderWidget {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget;
}

pub trait ApplyWidget {
  fn apply(&self, widget: &impl WidgetExt);
}
