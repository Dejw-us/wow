use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::action::Action;
use crate::context::Context;
use crate::display::TrySetText;
use crate::value::Value;
use gtk4::glib::{timeout_add_local, ControlFlow, WeakRef};
use gtk4::Widget;
use serde::de::Error;
use std::any::Any;
use std::rc::Rc;
use std::time::Duration;

pub struct RepeatAction {
  action: Action,
  delay: Duration,
}

impl TryFromRawAction for RepeatAction {
  fn try_from_raw_action<E: Error>(raw: RawAction) -> Result<Self, E> {
    let action: Action = serde_yaml::from_str(&raw.de_param(0)?).map_err(|e| E::custom(e))?;
    let delay = raw
      .de_param(1)?
      .parse()
      .map_err(|_| E::custom("Expected number"))?;

    Ok(Self {
      action,
      delay: Duration::from_millis(delay),
    })
  }
}

impl RunAction for RepeatAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    let action = self.action.clone_inner();
    let value = self.action.run(context.clone(), widget.clone());
    timeout_add_local(self.delay, move || {
      let value = action.run(context.clone(), widget.clone());
      if let Some(widget) = widget.upgrade() {
        widget.try_set_text(&value.to_string())
      }
      ControlFlow::Continue
    });
    value
  }
}
