use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;

pub struct NoneAction;

impl TryFromRawAction for NoneAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    Ok(Self)
  }
}

impl RunAction for NoneAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, _context: Rc<Context>, widget: WeakRef<gtk4::Widget>) -> Value {
    Value::None
  }
}
