use gtk4::glib::WeakRef;
use gtk4::{Button, Label};
use wow::peek::OptionPeek;
use wow::state::StateValue;
use wow::text::TextDisplay;

pub enum StateListener {
  Label(WeakRef<Label>),
  Button(WeakRef<Button>),
}

impl StateListener {
  pub fn run(&self, value: &StateValue) {
    match self {
      StateListener::Label(weak) => weak.upgrade().if_some(|l| l.set_text(&value.to_string())),
      StateListener::Button(weak) => weak.upgrade().if_some(|b| b.set_text(&value.to_string())),
    }
  }
}
