use crate::config::window::WindowConfig;
use crate::context::Context;
use crate::listener::AppListener;
use crate::state::{State, StateValue};
use crate::util::file::read_file_to_string;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, Cast, GtkWindowExt, ObjectExt};
use std::io::Write;
use std::ops::DerefMut;
use std::rc::Rc;

pub mod action;
pub mod config;
pub mod context;
pub mod error;
pub mod listener;
pub mod map;
pub mod peek;
pub mod state;
pub mod text;
pub mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let data = read_file_to_string("example.yml")?;
  let window: WindowConfig = serde_yaml::from_str(&data)?;
  println!("{}", data);
  println!("{:?}", window);
  AppListener::clear()?;
  let context = Rc::new(Context::new(
    map! {
      "message".to_string() => State::new(StateValue::Int(32))
    },
    map!("example".to_string() => window),
  ));
  let listener = AppListener::new()?;
  listener.start(context);
  Ok(())
}
