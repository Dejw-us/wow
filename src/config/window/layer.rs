use gtk4_layer_shell::Layer;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WindowLayer {
  Background,
  Bottom,
  Top,
  Overlay,
}

impl Into<Layer> for WindowLayer {
  fn into(self) -> Layer {
    match self {
      WindowLayer::Background => Layer::Background,
      WindowLayer::Bottom => Layer::Bottom,
      WindowLayer::Top => Layer::Top,
      WindowLayer::Overlay => Layer::Overlay,
    }
  }
}
