use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
  pub fields: HashMap<String, Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Field {
  Value(String),
  Object(Object),
}

impl Object {
  pub fn new() -> Self {
    Self {
      fields: HashMap::new(),
    }
  }

  pub fn get_field(&self, name: &str) -> Option<&Field> {
    self.fields.get(name)
  }

  pub fn set_field(&mut self, name: String, field: Field) {
    self.fields.insert(name, field);
  }
}
