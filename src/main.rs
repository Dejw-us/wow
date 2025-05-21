use std::env::set_var;
use std::fs::File;
use std::io::Read;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use window::WindowManager;

pub mod window;

fn main() {
  unsafe {
    set_var("GDK_BACKEND", "wayland");
  }

  let mut file = File::open("./example.yml").expect("Failed to open file");
  let window_manager = WindowManager::from_config(&mut file);
  println!("Window manager: {:?}", window_manager);

  gtk4::init().expect("Failed to init gtk4");

  let window = window_manager.window("test").unwrap();

  let mut css_file = File::open("./style.css").expect("Failed to open styles");
  let mut buf = String::new();
  css_file
    .read_to_string(&mut buf)
    .expect("Failed to read css");

  window.open(&buf);
}
