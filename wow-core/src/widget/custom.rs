use crate::action::Action;
use getset::Getters;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Getters, Debug, Deserialize)]
#[get = "pub"]
pub struct CustomConfig {
  name: String,
  params: Option<HashMap<String, Action>>,
}
