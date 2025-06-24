use crate::context::Context;
use crate::listener::message::Message;
use gtk4::prelude::{
  ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt, WidgetExt,
};
use gtk4::{glib, Application};
use std::fs::{metadata, remove_file};
use std::io;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::rc::Rc;
use std::thread::spawn;

pub mod message;

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

  pub fn start(self, context: Context) {
    let (sender, receiver) = async_channel::bounded(1);
    let app = Application::builder().application_id("me.dejw-us").build();
    let context = Rc::new(context);
    app.connect_activate(move |app| {
      let guard = app.hold();
      let receiver = receiver.clone();
      let app = app.clone();
      let context = context.clone();
      glib::spawn_future_local(async move {
        while let Ok(msg) = receiver.recv().await {
          println!("Received message {:?}", msg);
          match msg {
            Message::SetState(name, value) => {
              context.set_state(&name, value);
            }
            Message::OpenWindow(name) => {
              Context::open_window(context.clone(), &name, &app);
            }
            Message::CloseWindow(name) => {
              for window in app.windows() {
                if window.widget_name() == name {
                  window.destroy();
                }
              }
            }
          }
        }
        println!("Exiting");
        drop(guard);
      });
      println!("Starting");
    });

    spawn(move || {
      for stream in self.listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        let msg = Message::parse(&buf);
        sender.send_blocking(msg).unwrap();
      }
    });

    app.run();
  }
}
