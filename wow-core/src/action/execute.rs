use crate::action::raw::RawAction;
use crate::action::traits::{RunAction, TryFromRawAction};
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use gtk4::Widget;
use serde::de::Error;
use std::any::Any;
use std::process::Command;
use std::rc::Rc;

pub struct ExecuteAction {
  command: String,
}

impl TryFromRawAction for ExecuteAction {
  fn try_from_raw_action<E: Error>(action: RawAction) -> Result<Self, E> {
    let command = action.de_param(0)?;

    Ok(Self { command })
  }
}

impl RunAction for ExecuteAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>, widget: WeakRef<Widget>) -> Value {
    let split: Vec<_> = self.command.split(" ").collect();
    let mut command = Command::new(split[0]);

    println!("Running {}", self.command);
    println!("CMD name: {:?}", command.get_program());
    for split in split.iter().skip(1) {
      command.arg(split);
    }

    match command.output() {
      Ok(output) => {
        let text = String::from_utf8(output.stdout).expect("Couldn't convert output to utf8");
        Value::String(text)
      }
      Err(err) => {
        println!("Failed to run command {}", err);
        Value::None
      }
    }
  }
}
