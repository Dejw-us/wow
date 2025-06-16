use crate::display::TextDisplay;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::{Button, Label};
use wow_utils::option::IfSome;

pub enum StateListener {
  None,
  Label(WeakRef<Label>),
  Button(WeakRef<Button>),
}

impl StateListener {
  pub fn run(&self, value: &Value) {
    match self {
      StateListener::Label(weak) => weak.upgrade().if_some(|l| l.set_text(&value.to_string())),
      StateListener::Button(weak) => weak.upgrade().if_some(|b| b.set_text(&value.to_string())),
      StateListener::None => (),
    }
  }
}
