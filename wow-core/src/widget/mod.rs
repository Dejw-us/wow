pub mod button;
pub mod label;

use crate::context::Context;
use crate::state::listener::StateListener;
use crate::widget::button::ButtonConfig;
use crate::widget::label::LabelConfig;
use gtk4::glib::WeakRef;
use gtk4::prelude::{ObjectType, WidgetExt};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Widget {
  Label(LabelConfig),
  Button(ButtonConfig),
}

pub trait WidgetEssentials {
  type Widget: WidgetExt;

  fn build() -> Self::Widget;
  fn listener(widget: WeakRef<Self::Widget>) -> StateListener;
}

impl RenderWidget for Widget {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget {
    match self {
      Widget::Label(label) => label.render(context.clone()),
      Widget::Button(button) => button.render(context.clone()),
    }
  }
}

pub trait RenderWidget {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget;
}

pub trait ApplyWidget {
  fn apply(&self, widget: &impl WidgetExt, context: Rc<Context>);
}
