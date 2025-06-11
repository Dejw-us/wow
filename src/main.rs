use crate::context::Context;
use crate::listener::AppListener;

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

  let context = Context::load().expect("Failed to load context");

  AppListener::new()?.start(context);

  Ok(())
}
