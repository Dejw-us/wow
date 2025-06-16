use crate::action::RunAction;
use crate::context::Context;
use crate::value::Value;
use std::any::Any;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SetStateAction {
  name: String,
  value: Value,
}

impl RunAction for SetStateAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>) -> Value {
    context.set_state_value(&self.name, self.value.clone());
    Value::None
  }
}
