use crate::display::TrySetText;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::Widget;

pub enum StateListener {
  None,
  Widget(WeakRef<Widget>),
}

impl StateListener {
  pub fn run(&self, value: &Value) {
    match self {
      StateListener::Widget(widget) => {
        if let Some(widget) = widget.upgrade() {
          widget.try_set_text(&value.to_string());
        }
      }
      StateListener::None => (),
    }
  }
}
