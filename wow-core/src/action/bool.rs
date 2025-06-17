use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::Widget;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;

pub struct BoolAction {
  value: bool,
}

impl TryFromRawAction for BoolAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    let value: bool = action
      .de_param(0)?
      .parse()
      .map_err(|_| E::custom("Expected bool"))?;

    Ok(Self { value })
  }
}

impl RunAction for BoolAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, _context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    self.value.into()
  }
}
