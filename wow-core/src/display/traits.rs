use gtk4::prelude::{ButtonExt, Cast, IsA, ObjectType};
use gtk4::{Button, Label, Widget};

// --- Traits definitions ---

pub trait TrySetText {
  fn try_set_text(&self, text: &str);
}

pub trait TextDisplay: ObjectType + IsA<Widget> {
  fn get_text(&self) -> String;
  fn set_text(&self, text: &str);
}

// --- Traits implementations ---

impl TrySetText for Widget {
  fn try_set_text(&self, text: &str) {
    if let Some(button) = self.downcast_ref::<Button>() {
      button.set_label(text);
    } else if let Some(label) = self.downcast_ref::<Label>() {
      label.set_text(text);
    }
  }
}

impl TextDisplay for Button {
  fn get_text(&self) -> String {
    self.label().map(|l| l.to_string()).unwrap_or(String::new())
  }

  fn set_text(&self, text: &str) {
    self.set_label(text);
  }
}

impl TextDisplay for Label {
  fn get_text(&self) -> String {
    self.label().to_string()
  }

  fn set_text(&self, text: &str) {
    self.set_label(text);
  }
}
