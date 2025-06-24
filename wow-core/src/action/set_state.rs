use crate::action::traits::TryFromRawAction;
use crate::action::{Action, RawAction, RunAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use serde::de;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug)]
pub struct SetStateAction {
  name: String,
  action: Action,
}

impl TryFromRawAction for SetStateAction {
  fn try_from_raw_action<E: de::Error>(action: RawAction) -> Result<Self, E> {
    let name = action.de_param(0)?;
    let value = action.de_param(1)?;

    Ok(Self {
      name,
      action: serde_yaml::from_str(&value).map_err(de::Error::custom)?,
    })
  }
}

impl RunAction for SetStateAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>, widget: WeakRef<gtk4::Widget>) -> Value {
    let value = self.action.run(context.clone(), widget);
    context.set_state(&self.name, value.clone());
    value
  }
}
