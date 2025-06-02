use crate::context::Context;
use crate::state::StateValue;
use std::rc::Rc;

pub enum ActionResult {
  String(String),
  None,
}

#[derive(Clone)]
pub enum Action {
  Log(String),
  SetState(String, StateValue),
  None,
}

impl Action {
  pub fn run(&self, context: Rc<Context>) -> ActionResult {
    match self {
      Action::Log(text) => {
        println!("{}", text);
        ActionResult::None
      }
      Action::SetState(name, value) => {
        context.set_state_value(name, value.clone());
        println!("Set {} to {}", name, value);
        ActionResult::None
      }
      Action::None => ActionResult::None,
    }
  }
}
