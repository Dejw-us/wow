use std::rc::Rc;

use serde::Deserialize;

use crate::error;

#[derive(Clone)]
pub struct Action(Rc<dyn Fn() -> error::Result<()> + 'static>);

impl Action {
  pub fn clone_inner(&self) -> Rc<dyn Fn() -> error::Result<()> + 'static> {
    self.0.clone()
  }
}

impl<'de> Deserialize<'de> for Action {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let raw_action = String::deserialize(deserializer)?;
    let action = move || {
      println!("action: {}", raw_action);
      Ok(())
    };
    Ok(Action(Rc::new(action)))
  }
}
