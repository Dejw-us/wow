use crate::config::widget::button::ButtonConfig;
use crate::config::widget::label::LabelConfig;
use crate::context::Context;
use std::rc::Rc;

pub mod button;
pub mod label;

pub enum Widget {
  Label(LabelConfig),
  Button(ButtonConfig),
}

impl Render for Widget {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget {
    match self {
      Widget::Label(label) => label.render(context),
      Widget::Button(button) => button.render(context),
    }
  }
}

pub trait Render {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget;
}
