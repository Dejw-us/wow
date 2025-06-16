use crate::action::log::LogAction;
use crate::context::Context;
use crate::functions::{Chars, Mappings};
use crate::value::Value;
use crate::widget::ApplyWidget;
use getset::Getters;
use gtk4::prelude::{ButtonExt, Cast, WidgetExt};
use gtk4::Button;
use serde::{de, Deserialize, Deserializer};
use std::any::Any;
use std::fmt::{Debug, Display};
use std::rc::Rc;

pub mod log;
pub mod request;
pub mod set_state;

pub trait TryFromRawAction: Sized {
  fn try_from_raw_action<E: de::Error>(action: RawAction) -> Result<Self, E>;
}

pub trait RunAction: Any {
  fn as_any(&self) -> &dyn Any;
  fn run(&self, context: Rc<Context>) -> Value;
}

pub struct Action {
  pub inner: Rc<dyn RunAction>,
}

impl Action {
  pub fn clone_inner(&self) -> Rc<dyn RunAction> {
    self.inner.clone()
  }
}

impl Debug for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Action")
  }
}

impl RunAction for Action {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>) -> Value {
    self.inner.run(context)
  }
}

#[derive(Getters)]
pub struct RawAction {
  #[get = "pub"]
  name: String,
  params: Vec<String>,
}

impl RawAction {
  pub fn param(&self, i: usize) -> Option<&String> {
    self.params.get(i)
  }

  pub fn de_param<E: de::Error>(&self, i: usize) -> Result<String, E> {
    match self.param(i) {
      None => Err(E::custom("param not found")),
      Some(p) => Ok(p.to_owned()),
    }
  }

  pub fn parse(s: &str) -> Result<Self, String> {
    let s = match s.strip_prefix("~") {
      None => return Err("Actions need to start with ~".into()),
      Some(s) => s,
    };

    match s.find("(") {
      None => return Err("Invalid syntax. Expected ()".into()),
      Some(i) => {
        let name = &s[..i];
        let params = &s[(i + 1)..];
        let params: String = params[..params.len() - 1]
          .chars()
          .filter(Chars::not_whitespace)
          .collect();
        let params: Vec<String> = params.split(",").map(Mappings::into).collect();

        Ok(RawAction {
          name: name.to_string(),
          params,
        })
      }
    }
  }
}

impl ApplyWidget for Action {
  fn apply(&self, widget: &impl WidgetExt, context: Rc<Context>) {
    let widget = widget.upcast_ref::<gtk4::Widget>();
    if let Some(button) = widget.downcast_ref::<Button>() {
      let inner = self.inner.clone();
      button.connect_clicked(move |_| {
        inner.run(context.clone());
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
    let raw_action = RawAction::parse(&s).map_err(serde::de::Error::custom)?;

    match raw_action.name.as_str() {
      "log" => Ok(Action {
        inner: Rc::new(LogAction::try_from_raw_action(raw_action)?),
      }),
      _ => Err(serde::de::Error::custom(format!(
        "Invalid action name: {}",
        raw_action.name
      ))),
    }
  }
}
