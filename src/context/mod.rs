use crate::state::{State, StateValue};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
use gtk4::{glib, Application, Window};
use std::collections::HashMap;
use std::fs::{metadata, remove_file};
use std::io;
use std::os::unix::net::UnixListener;
use std::thread::spawn;

const LISTENER_PATH: &str = "/tmp/wow.sock";
const APP_ID: &str = "github.wow";

pub struct Context {
  states: HashMap<String, State>,
  app: Application,
  listener: UnixListener,
}
impl Context {
  pub fn new() -> io::Result<Self> {
    let listener = UnixListener::bind(LISTENER_PATH)?;
    let app = Application::builder().application_id(APP_ID).build();
    Ok(Context {
      listener,
      app,
      states: HashMap::new(),
    })
  }

  pub fn clear() -> io::Result<()> {
    if metadata(LISTENER_PATH).is_ok() {
      remove_file(LISTENER_PATH)
    } else {
      Ok(())
    }
  }

  pub fn get_state_value(&self, key: &str) -> Option<&StateValue> {
    self.states.get(key).map(|s| s.get())
  }

  pub fn set_state(&mut self, name: &str, value: StateValue) {
    match self.states.get(name) {
      None => {
        self.states.insert(name.to_string(), State::new(value));
      }
      Some(mut state) => {
        state.set(value);
      }
    };
  }

  pub fn start(self) {
    let (sender, receiver) = async_channel::bounded(1);

    self.app.connect_activate(move |app| {
      let guard = app.hold();
      let receiver = receiver.clone();
      let app = app.clone();
      glib::spawn_future_local(async move {
        while let Ok(_) = receiver.recv().await {
          println!("Received message");
          let window = Window::builder().application(&app).build();
          window.present();
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

    self.app.run();
  }
}
