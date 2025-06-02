use gtk4_layer_shell::Edge;

#[derive(Clone)]
pub enum WindowAnchor {
  Top,
  Bottom,
  Left,
  Right,
}

impl Into<Edge> for WindowAnchor {
  fn into(self) -> Edge {
    match self {
      WindowAnchor::Top => Edge::Top,
      WindowAnchor::Bottom => Edge::Bottom,
      WindowAnchor::Left => Edge::Left,
      WindowAnchor::Right => Edge::Right,
    }
  }
}
