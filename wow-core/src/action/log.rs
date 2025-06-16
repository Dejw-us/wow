use crate::action::RunAction;
use crate::context::Context;
use crate::value::Value;
use derive_new::new;
use getset::Getters;
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, new, Default, Getters)]
#[get = "pub"]
pub struct LogAction {
  message: String,
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
