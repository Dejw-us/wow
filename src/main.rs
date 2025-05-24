use std::{collections::HashMap, fs::File, io::Read};

use widget::{RawWidget, Widget};
use window::{Window, WindowBuilder};
pub mod error;
pub mod widget;
pub mod window;

fn main() {
  gtk4::init().expect("Failed to init gtk");

  let mut file = File::open("./example.yml").expect("Failed to open file");
  let mut buf = String::new();
  file.read_to_string(&mut buf).expect("Failed to read file");
  let widget: HashMap<String, RawWidget> =
    serde_yaml::from_str(&buf).expect("Failed to deserialize");
  let widget = widget.get("test").unwrap();

  let window = WindowBuilder::default()
    .id("me.dawid".to_string())
    .child(Some(widget.into()))
    .build()
    .expect("Failed to build window");

  window.open(true);
}
