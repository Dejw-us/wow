use crate::context::Context;
use crate::state::listener::StateListener;
use chrono::Local;
use gtk4::glib::{timeout_add_local, ControlFlow, WeakRef};
use gtk4::prelude::{ButtonExt, ObjectType};
use gtk4::{Button, Label};
use serde::{Deserialize, Deserializer};
use std::time::Duration;
use wow_utils::option::IfSome;

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

#[derive(Debug)]
pub enum Text {
  Text(String),
  State(String),
  Clock(String, u64),
}

impl<'de> Deserialize<'de> for Text {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;

    match s {
      s if s.eq("CLOCK") => Ok(Self::Clock("%Y-%m-%d %H:%M:%S".to_string(), 1000)),
      s if s.starts_with("$") => Ok(Self::State(s[1..].to_string())),
      _ => Ok(Self::Text(s)),
    }
  }
}

impl Text {
  pub fn convert(
    &self,
    context: &Context,
    listener: impl Fn() -> StateListener,
    weak: WeakRef<impl TextDisplay>,
  ) -> String {
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
        let time_local = now.format(&format).to_string();
        let format = format.to_string();
        timeout_add_local(Duration::from_millis(update_rate.clone()), move || {
          let now = Local::now();
          let time = now.format(&format).to_string();
          weak.upgrade().if_some(|label| label.set_text(&time));
          ControlFlow::Continue
        });
        time_local
      }
    }
  }
}
