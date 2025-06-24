use crate::action::raw::RawAction;
use crate::action::traits::RunAction;
use crate::context::Context;
use crate::display::traits::TrySetText;
use crate::display::utils::value_to_string;
use crate::state::listener::StateListener;
use crate::widget::ApplyWidget;
use gtk4::prelude::ObjectExt;
use gtk4::Widget;
use log::error;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::rc::Rc;

#[derive(Debug)]
pub enum Text {
  Action(crate::action::Action),
  Text(String),
  State(String),
}

impl TryFrom<String> for Text {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value {
      s if s.starts_with("$") => Ok(Self::State(s[1..].to_string())),
      s if s.starts_with("~") => {
        let raw = RawAction::parse(s.as_str())?;
        let action: crate::action::Action = raw
          .try_into()
          .map_err(|e| crate::Error::Message(format!("{}", e)))?;
        Ok(Self::Action(action))
      }
      _ => Ok(Self::Text(value.to_string())),
    }
  }
}

impl<'de> Deserialize<'de> for Text {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = value_to_string(Value::deserialize(deserializer)?);
    match Self::try_from(value) {
      Ok(value) => Ok(value),
      Err(err) => Err(serde::de::Error::custom(format!("{}", err))),
    }
  }
}

impl ApplyWidget for Text {
  fn apply(&self, widget: &Widget, context: Rc<Context>) {
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
          error!("Failed to find state {}", state_name);
        }
      }
    };
  }
}
