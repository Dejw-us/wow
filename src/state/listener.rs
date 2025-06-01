use crate::peek::PeekOption;
use crate::state::StateValue;
use gtk4::glib::WeakRef;
use gtk4::Label;

pub enum StateListener {
  Label(WeakRef<Label>),
}

impl StateListener {
  pub fn run(&self, value: &StateValue) {
    match self {
      StateListener::Label(label) => label
        .upgrade()
        .peek(|label| label.set_label(&value.to_string())),
    }
  }
}
