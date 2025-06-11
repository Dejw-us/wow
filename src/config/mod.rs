use gtk4::prelude::WidgetExt;

pub mod align;
pub mod geometry;
pub mod style;
pub mod widget;
pub mod window;

pub trait ApplyWidget {
  fn apply(&self, widget: &impl WidgetExt);
}
