use crate::context::Context;
use crate::state::listener::StateListener;
use chrono::Local;
use gtk4::prelude::{ButtonExt, ObjectType};
use gtk4::{Button, Label};

pub trait TextDisplay: ObjectType {
  fn get_text(&self) -> String;
  fn set_text(&self, text: &str);
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

pub enum Text {
  Text(String),
  State(String),
  Clock(String, i32),
}

impl Text {
  pub fn convert(&self, context: &Context, listener: impl Fn() -> StateListener) -> String {
    match self {
      Text::Text(text) => text.into(),
      Text::State(state_name) => {
        if let Some(state) = context.get_state(state_name) {
          state.subscribe(listener());
          state.get().to_string()
        } else {
          panic!("Failed to find state");
        }
      }
      Text::Clock(format, update_rate) => {
        let now = Local::now();
        now.format(&format).to_string()
      }
    }
  }
}
