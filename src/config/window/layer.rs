use gtk4_layer_shell::Layer;

#[derive(Clone)]
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
