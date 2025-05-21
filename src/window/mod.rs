use std::{collections::HashMap, fs::File, io::Read};

pub struct Window {
  title: String,
}

pub struct WindowManager {
  windows: HashMap<String, Window>,
}

impl WindowManager {
  pub fn from_config(config: &mut File) -> Self {
    let mut buf = String::new();
    config
      .read_to_string(&mut buf)
      .expect("Failed to read file");
    let windows: HashMap<String, Window> =
      serde_yaml::from_str(&buf).expect("Failed to parse yaml");
    Self { windows }
  }
}
