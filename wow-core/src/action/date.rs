use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use chrono::Local;
use gtk4::glib::WeakRef;
use gtk4::Widget;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;

pub struct DateAction {
  format: String,
}

impl TryFromRawAction for DateAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    let format = action.de_param(0)?;
    Ok(Self { format })
  }
}

impl RunAction for DateAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, _context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    let now = Local::now();
    let time_local = now.format(&self.format).to_string();

    Value::String(time_local)
  }
}
