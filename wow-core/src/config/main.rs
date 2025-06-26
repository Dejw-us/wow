use getset::Getters;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Getters)]
pub struct Main {
  states: HashMap<String, serde_yaml::Value>,
}
