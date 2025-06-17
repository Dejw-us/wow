use crate::action::RunAction;
use crate::context::Context;
use crate::value::Value;
use gtk4::glib::WeakRef;
use std::any::Any;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum HttpMethod {
  Get,
  Post,
}

#[derive(Clone, Debug)]
pub struct RequestAction {
  url: String,
  method: HttpMethod,
}

impl RunAction for RequestAction {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn run(&self, context: Rc<Context>, widget: WeakRef<gtk4::Widget>) -> Value {
    Value::String("test".into())
  }
}
