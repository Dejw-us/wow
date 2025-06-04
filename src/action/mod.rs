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
      s if s.starts_with("$log") => Ok(Action::Log(s[5..].to_string())),
      s if s.starts_with("$set") => {
        let split: Vec<_> = s.splitn(3, &s).into_iter().collect();
        Ok(Action::SetState(
          split[1].to_string(),
          StateValue::String(split[2].to_string()),
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
