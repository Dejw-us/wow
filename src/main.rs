use log::info;
use wow_core::context::Context;
use wow_core::listener::AppListener;

fn main() {
  AppListener::clear().expect("Failed to clear app");
  log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
  info!("Starting Wow");
  let context = Context::load().expect("Failed to load context");
  AppListener::new()
    .expect("Failed to create app")
    .start(context);
}
