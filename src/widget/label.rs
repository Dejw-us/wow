use serde::Deserialize;

use super::state::State;

#[derive(Clone, Default, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WidgetLabel {
  #[default]
  Empty,
  Exact {
    label: String,
  },
  State {
    name: String,
  },
}
