use crate::context::Context;
use crate::listener::AppListener;
use crate::state::{State, StateValue};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, Cast, GtkWindowExt, ObjectExt};
use std::io::Write;
use std::ops::DerefMut;
use std::rc::Rc;

pub mod config;
pub mod context;
pub mod error;
pub mod listener;
pub mod map;
pub mod peek;
pub mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  AppListener::clear()?;
  let context = Rc::new(Context::with_states(map! {
    "test".to_string() => State::new(StateValue::Int(32))
  }));
  let listener = AppListener::new()?;
  listener.start(context);
  Ok(())
}
