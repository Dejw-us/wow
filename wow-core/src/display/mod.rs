use crate::action::raw::RawAction;
use crate::action::traits::RunAction;
use crate::context::Context;
use crate::state::listener::StateListener;
use crate::widget::ApplyWidget;
use gtk4::prelude::{ButtonExt, Cast, IsA, ObjectExt, ObjectType};
use gtk4::{Button, Label, Widget};
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::rc::Rc;

pub trait TrySetText {
  fn try_set_text(&self, text: &str);
}

impl TrySetText for Widget {
  fn try_set_text(&self, text: &str) {
    if let Some(button) = self.downcast_ref::<Button>() {
      button.set_label(text);
    } else if let Some(label) = self.downcast_ref::<Label>() {
      label.set_text(text);
    }
  }
}

pub trait TextDisplay: ObjectType + IsA<gtk4::Widget> {
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
  Action(crate::action::Action),
  Text(String),
  State(String),
}

impl<'de> Deserialize<'de> for Text {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = serde_yaml::Value::deserialize(deserializer)?;
    // TODO cleanup
    let s = match s {
      Value::Null => String::new(),
      Value::Bool(bool) => bool.to_string(),
      Value::Number(num) => num.to_string(),
      Value::String(string) => string,
      Value::Sequence(_) => String::new(),
      Value::Mapping(_) => String::new(),
      Value::Tagged(_) => String::new(),
    };
    match s {
      s if s.starts_with("$") => Ok(Self::State(s[1..].to_string())),
      s if s.starts_with("~") => {
        let raw = RawAction::parse(s.as_str()).map_err(serde::de::Error::custom)?;
        let action: crate::action::Action =
          raw.try_into().map_err(|e| serde::de::Error::custom(e))?;
        Ok(Self::Action(action))
      }
      _ => Ok(Self::Text(s.to_string())),
    }
  }
}

impl ApplyWidget for Text {
  fn apply(&self, widget: &gtk4::Widget, context: Rc<Context>) {
    let weak = widget.downgrade();
    match self {
      Text::Action(action) => {
        let value = action.run(context.clone(), weak.clone());
        widget.try_set_text(&value.to_string());
      }
      Text::Text(text) => widget.try_set_text(text),
      Text::State(state_name) => {
        if let Some(state) = context.get_state(state_name) {
          state.subscribe(StateListener::Widget(weak));
          widget.try_set_text(&state.get().to_string());
        } else {
          panic!("Failed to find state {}", state_name);
        }
      }
    };
  }
}
