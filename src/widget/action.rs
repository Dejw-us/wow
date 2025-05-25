use std::{fmt::Debug, marker::PhantomData, rc::Rc, str::FromStr};

use serde::Deserialize;

use crate::error;

use super::{
  container::RawContainer,
  state::{StateValue, StateWidget, WidgetStates},
};

#[derive(Clone)]
pub enum Action {
  None,
  SetState { name: String, value: StateValue },
}

impl Debug for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Action").finish()
  }
}

impl Action {
  pub fn run(&self, states: &WidgetStates) {
    match self {
      Action::None => (),
      Action::SetState { name, value } => self.run_set_state(states, &name, value),
    }
  }

  fn run_set_state(&self, states: &WidgetStates, name: &str, value: &StateValue) {
    states.set(name, value.clone());
  }
}

impl<'de> Deserialize<'de> for Action {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match Action::from_str(&s) {
      Ok(action) => Ok(action),
      Err(message) => {
        println!("Failed to deserialize: {}", message);
        Ok(Action::None)
      }
    }
  }
}

impl FromStr for Action {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.trim();
    let i = s.find("(").unwrap_or(0);
    let name = &s[0..i];
    let args = s
      .strip_prefix(&format!("{}(", name))
      .expect("Failed to strip prefix")
      .strip_suffix(")")
      .expect("Failed to strip suffix");
    let args: Vec<&str> = args.split(",").into_iter().collect();
    Ok(match name {
      "setState" => Action::SetState {
        name: args[0].to_string(),
        value: StateValue::String(args[1].to_string()),
      },
      _ => panic!("Failed to match"),
    })
  }
}
