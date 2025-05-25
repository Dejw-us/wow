use container::{Container, RawContainer};
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

#[derive(Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RawWidget {
  Container(RawContainer),
}

// impl Into<Widget> for &RawWidget {
//   fn into(self) -> Widget {
//     match self {
//       RawWidget::Container(container) => Widget::Container(container.clone()),
//     }
//   }
// }

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
