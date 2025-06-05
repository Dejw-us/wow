use crate::config::window::WindowConfig;
use crate::context::Context;
use crate::listener::AppListener;
use crate::state::StateValue;
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
  let data = read_file_to_string("example.yml")?;
  let window: WindowConfig = serde_yaml::from_str(&data)?;
  let states = serde_yaml::from_str::<HashMap<String, Value>>(&data)?;
  let states = states
    .iter()
    .filter(|(key, _)| key.starts_with("$"))
    .collect::<Vec<_>>();
  println!("{}", data);
  println!("{:?}", window);
  AppListener::clear()?;
  let context = Rc::new(Context::new(
    HashMap::new(),
    map!("example".to_string() => window),
  ));
  for (key, value) in states {
    let value = match value {
      Value::String(string) => StateValue::String(string.to_string()),
      Value::Number(number) => {
        if number.is_i64() {
          StateValue::Int(number.as_i64().unwrap())
        } else if number.is_f64() {
          StateValue::Float(number.as_f64().unwrap())
        } else {
          panic!("Failed to convert number to StateValue")
        }
      }
      _ => panic!("Invalid value format"),
    };
    context.set_state_value(&key[1..], value);
  }
  let listener = AppListener::new()?;
  listener.start(context);
  Ok(())
}
