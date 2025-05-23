use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
  Horizontal,
  Vertical,
}

impl From<&Orientation> for gtk4::Orientation {
  fn from(value: &Orientation) -> Self {
    match value {
      Orientation::Horizontal => gtk4::Orientation::Horizontal,
      Orientation::Vertical => gtk4::Orientation::Vertical,
    }
  }
}
