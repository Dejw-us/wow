use serde::Deserialize;

use super::state::State;

#[derive(Clone, Default, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WidgetLabel {
  #[default]
  Empty,
  Exact(String),
  State(String),
}
