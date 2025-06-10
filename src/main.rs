use crate::config::window::WindowConfig;
use crate::context::Context;
use crate::listener::AppListener;
use crate::util::file::read_file_to_string;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, Cast, GtkWindowExt, ObjectExt};
use serde_yaml::Value;
use std::collections::HashMap;
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
  AppListener::clear()?;

  let data = read_file_to_string("example.yml")?;
  let window: WindowConfig = serde_yaml::from_str(&data)?;
  let states = serde_yaml::from_str::<HashMap<String, Value>>(&data)?;

  let context = Rc::new(Context::new(
    HashMap::new(),
    map!("example".to_string() => window),
  ));

  let states = states
    .iter()
    .filter(|(key, _)| key.starts_with("$"))
    .collect::<Vec<_>>();

  for (key, value) in states {
    match value.try_into() {
      Ok(value) => context.set_state_value(&key[1..], value),
      Err(err) => println!("{}", err),
    }
  }

  AppListener::new()?.start(context);

  Ok(())
}
