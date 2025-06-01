use crate::config::text::Text;
use crate::config::widget::label::LabelConfig;
use crate::config::window::WindowConfig;
use crate::context::Context;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::{glib, Application};
use std::fs::{metadata, remove_file};
use std::io;
use std::os::unix::net::UnixListener;
use std::rc::Rc;
use std::thread::spawn;

const LISTENER_PATH: &str = "/tmp/wow.sock";

pub struct AppListener {
  listener: UnixListener,
}

impl AppListener {
  pub fn new() -> io::Result<Self> {
    let listener = UnixListener::bind(LISTENER_PATH)?;
    Ok(Self { listener })
  }

  pub fn clear() -> io::Result<()> {
    if metadata(LISTENER_PATH).is_ok() {
      remove_file(LISTENER_PATH)
    } else {
      Ok(())
    }
  }

  pub fn start(self, context: Rc<Context>) {
    let (sender, receiver) = async_channel::bounded(1);
    let app = Application::builder().application_id("me.dejw-us").build();

    app.connect_activate(move |app| {
      let guard = app.hold();
      let receiver = receiver.clone();
      let app = app.clone();
      let context = context.clone();
      glib::spawn_future_local(async move {
        while let Ok(_) = receiver.recv().await {
          println!("Received message");
          let window = WindowConfig::with_child(
            LabelConfig::with_label(Text::State("test".to_string())).into(),
          );
          window.render(&app, context.as_ref());
        }
        println!("Exiting");
        drop(guard);
      });
      println!("Starting");
    });

    spawn(move || {
      println!("Listening on {}", LISTENER_PATH);
      for stream in self.listener.incoming() {
        println!("Accepted connection");
        sender.send_blocking(true).unwrap();
      }
    });

    app.run();
  }
}
