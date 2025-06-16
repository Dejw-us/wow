use crate::action::request::RequestAction;
use crate::context::Context;
use crate::object::Object;
use crate::state::StateValue;
use crate::widget::ApplyWidget;
use gtk4::prelude::{ButtonExt, Cast, WidgetExt};
use gtk4::Button;
use serde::{Deserialize, Deserializer};
use std::rc::Rc;

pub mod request;

pub enum ReturnAction {
  Request(RequestAction),
}

pub enum UnitAction {
  Log(String),
  SetState(String, StateValue),
}

pub enum Action {
  Return(ReturnAction),
  Unit(UnitAction),
}

pub enum ActionResult {
  Object(Object),
  String(String),
  None,
}

#[derive(Debug, Clone)]
pub struct Path(Vec<String>);

impl ApplyWidget for Action {
  fn apply(&self, widget: &impl WidgetExt, context: Rc<Context>) {
    let widget = widget.upcast_ref::<gtk4::Widget>();
    if let Some(button) = widget.downcast_ref::<Button>() {
      let action = self.clone();
      button.connect_clicked(move |_| {
        action.run(context.clone());
      });
    }
  }
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
