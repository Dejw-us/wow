pub mod button;
pub mod container;
pub mod label;

use crate::context::Context;
use crate::widget::button::ButtonConfig;
use crate::widget::container::ContainerConfig;
use crate::widget::label::LabelConfig;
use gtk4::prelude::{BoxExt, Cast, GskRendererExt, ObjectType, WidgetExt};
use serde::Deserialize;
use std::fmt::Debug;
use std::rc::Rc;
use wow_utils::option::IfSome;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Widget {
  Label(LabelConfig),
  Button(ButtonConfig),
  Container(ContainerConfig),
  Custom { name: String },
}

impl ApplyWidget for Vec<Widget> {
  fn apply(&self, widget: &gtk4::Widget, context: Rc<Context>) {
    if let Some(container) = widget.downcast_ref::<gtk4::Box>() {
      for child in self.iter() {
        container.append(&child.render(context.clone()));
      }
    }
  }
}

pub trait WidgetEssentials {
  fn build() -> gtk4::Widget;
}

impl RenderWidget for Widget {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget {
    match self {
      Widget::Label(label) => label.render(context.clone()),
      Widget::Button(button) => button.render(context.clone()),
      Widget::Container(container) => container.render(context.clone()),
      Widget::Custom { name } => {
        if let Some(widget) = context.get_custom_widget(name) {
          widget.render(context.clone())
        } else {
          panic!("Cannot render widget with name {}", name)
        }
      }
    }
  }
}

pub trait RenderWidget: Debug {
  fn render(&self, context: Rc<Context>) -> gtk4::Widget;
}

pub trait ApplyWidget: Debug {
  fn apply(&self, widget: &gtk4::Widget, context: Rc<Context>);
}
