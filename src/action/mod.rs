use crate::context::Context;
use crate::state::StateValue;
use serde::{Deserialize, Deserializer};
use std::rc::Rc;

pub enum ActionResult {
  String(String),
  None,
}

#[derive(Clone, Debug)]
pub enum Action {
  Log(String),
  SetState(String, StateValue),
  None,
}

impl<'de> Deserialize<'de> for Action {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;

    match s {
      s if s.starts_with("LOG") => Ok(Action::Log(s[4..].to_string())),
      s if s.starts_with("$") => {
        println!("fdsfdsfdsfdsf");
        let s = s.trim().replace(" ", "");
        let i = s.find("=").unwrap();
        let name = &s[1..i];
        let value = &s[i + 1..];
        println!("${}={}", name, value);
        Ok(Action::SetState(
          name.to_string(),
          StateValue::String(value.to_string()),
        ))
      }
      _ => Ok(Action::None),
    }
  }
}

impl Action {
  pub fn run(&self, context: Rc<Context>) -> ActionResult {
    match self {
      Action::Log(text) => {
        println!("{}", text);
        ActionResult::None
      }
      Action::SetState(name, value) => {
        context.set_state_value(name, value.clone());
        println!("Set {} to {}", name, value);
        ActionResult::None
      }
      Action::None => ActionResult::None,
    }
  }
}
