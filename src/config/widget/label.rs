use crate::config::widget::Render;
use crate::context::Context;
use crate::state::listener::StateListener;
use crate::text::Text;
use gtk4::prelude::{Cast, ObjectExt};
use gtk4::Label;
use std::rc::Rc;

pub struct LabelConfig {
  label: Text,
}

impl Render for LabelConfig {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget {
    let label = Label::builder().build();
    let label_name = self.label.convert(
      context.as_ref(),
      || StateListener::Label(label.downgrade()),
      label.downgrade(),
    );
    label.set_label(&label_name);
    label.upcast()
  }
}

impl LabelConfig {
  pub fn with_label(label: Text) -> Self {
    LabelConfig { label }
  }
}
