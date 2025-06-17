use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use crate::widget::ApplyWidget;
use gtk4::glib::WeakRef;
use gtk4::prelude::{ButtonExt, Cast, ObjectExt, WidgetExt};
use gtk4::{Button, Widget};
use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::fmt::{Debug, Display};
use std::rc::Rc;

pub mod bool;
pub mod date;
pub mod execute;
pub mod float;
pub mod int;
pub mod log;
pub mod none;
pub mod raw;
pub mod repeat;
pub mod request;
pub mod set_state;
pub mod string;
pub mod traits;
mod utils;

pub struct Action {
  pub inner: Rc<dyn RunAction>,
}

impl Action {
  pub fn clone_inner(&self) -> Rc<dyn RunAction> {
    self.inner.clone()
  }

  pub fn new(inner: impl RunAction) -> Self {
    Self {
      inner: Rc::new(inner),
    }
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

  fn run(&self, context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    self.inner.run(context, widget)
  }
}

impl ApplyWidget for Action {
  fn apply(&self, widget: &Widget, context: Rc<Context>) {
    let widget = widget.upcast_ref::<gtk4::Widget>();
    let weak_ref = widget.downgrade();
    if let Some(button) = widget.downcast_ref::<Button>() {
      let inner = self.clone_inner();
      button.connect_clicked(move |_| {
        inner.run(context.clone(), weak_ref.clone());
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
    let action = raw_action
      .try_into()
      .map_err(|e| serde::de::Error::custom(e))?;
    Ok(action)
  }
}
