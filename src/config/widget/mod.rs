use crate::config::widget::label::LabelConfig;
use crate::context::Context;

pub mod label;

pub enum Widget {
  Label(LabelConfig),
}

impl Widget {
  pub fn render(&self, context: &Context) -> gtk4::Widget {
    match self {
      Widget::Label(label) => label.render(context),
    }
  }
}
