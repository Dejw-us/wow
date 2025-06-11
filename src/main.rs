use wow_core::context::Context;
use wow_core::listener::AppListener;

fn main() {
  AppListener::clear().expect("Failed to clear app");

  let context = Context::load().expect("Failed to load context");
  AppListener::new()
    .expect("Failed to create app")
    .start(context);
}
