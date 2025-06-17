use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
  pub fields: HashMap<String, Value>,
}

impl Object {
  pub fn new() -> Self {
    Self {
      fields: HashMap::new(),
    }
  }

  pub fn get_field(&self, name: &str) -> Option<&Value> {
    self.fields.get(name)
  }

  pub fn set_field(&mut self, name: String, field: Value) {
    self.fields.insert(name, field);
  }
}
