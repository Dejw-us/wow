use crate::context::Context;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, Cast, GtkWindowExt, ObjectExt};
use std::io::Write;
pub mod context;
pub mod error;
pub mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  Context::clear()?;
  let context = Context::new()?;
  context.start();
  Ok(())
}
