use crate::action::RawAction;
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::Widget;
use serde::de;
use std::any::Any;
use std::rc::Rc;

pub trait TryFromRawAction: Sized {
  fn try_from_raw_action<E: de::Error>(action: RawAction) -> Result<Self, E>;
}

pub trait RunAction: Any {
  fn as_any(&self) -> &dyn Any;
  fn run(&self, context: Rc<Context>, widget: WeakRef<Widget>) -> Value;
}
