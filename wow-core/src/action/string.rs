use crate::action::traits::TryFromRawAction;
use crate::action::{RawAction, RunAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;

pub struct StringAction {
  value: Value,
}

impl TryFromRawAction for StringAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    let value = action.de_param(0)?.into();
    Ok(Self { value })
  }
}

impl RunAction for StringAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>, widget: WeakRef<gtk4::Widget>) -> Value {
    self.value.clone()
  }
}
