use crate::context::Context;
use crate::widget::ApplyWidget;
use gtk4::prelude::{BoxExt, Cast};
use gtk4::Widget;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug)]
pub struct Spacing(i32);

impl ApplyWidget for Spacing {
  fn apply(&self, widget: &Widget, context: Rc<Context>) {
    if let Some(container) = widget.downcast_ref::<gtk4::Box>() {
      println!("Setting spacing to {:?}", self.0);
      container.set_spacing(self.0);
    }
  }
}
