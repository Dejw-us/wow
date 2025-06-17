use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::Widget;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;

pub struct IntAction {
  value: i64,
}

impl TryFromRawAction for IntAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    let value: i64 = action
      .de_param(0)?
      .parse()
      .map_err(|_| E::custom("Expected int"))?;
    Ok(Self {
      value: value.into(),
    })
  }
}

impl RunAction for IntAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, _context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    self.value.into()
  }
}
