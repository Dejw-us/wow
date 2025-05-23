use container::Container;
use serde::Deserialize;

pub mod action;
pub mod container;
pub mod label;
pub mod margin;
pub mod orientation;
pub mod state;

#[derive(Clone, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Widget {
  Container(Container),
}

impl WidgetRender for Widget {
  fn render(&self) -> gtk4::Widget {
    match self {
      Widget::Container(container) => container.render(),
    }
  }
}

pub trait WidgetRender {
  fn render(&self) -> gtk4::Widget;
}
