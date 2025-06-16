use crate::action::{RawAction, RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use derive_new::new;
use getset::Getters;
use serde::de;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, new, Default, Getters)]
#[get = "pub"]
pub struct LogAction {
  message: String,
}

impl TryFromRawAction for LogAction {
  fn try_from_raw_action<E: de::Error>(value: RawAction) -> Result<Self, E> {
    Ok(Self {
      message: value.de_param::<E>(0)?,
    })
  }
}

impl RunAction for LogAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, _context: Rc<Context>) -> Value {
    println!("{}", self.message);
    Value::None
  }
}
