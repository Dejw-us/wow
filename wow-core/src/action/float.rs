use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::Widget;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;

pub struct FloatAction {
  value: f64,
}

impl TryFromRawAction for FloatAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    let value: f64 = action
      .de_param(0)?
      .parse()
      .map_err(|_| E::custom("expected a float"))?;
    Ok(Self { value })
  }
}

impl RunAction for FloatAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, _context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    self.value.into()
  }
}
