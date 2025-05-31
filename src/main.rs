use gtk4::prelude::{ApplicationExt, ApplicationExtManual, Cast, GtkWindowExt, ObjectExt};
use gtk4::{glib, Application, Window};
use std::fs::{metadata, remove_file};
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::thread::spawn;

pub mod error;

fn main() {
  let socket_path = "/tmp/wow.sock";

  if (metadata(socket_path).is_ok()) {
    remove_file(socket_path).expect("Failed to remove old socket");
  }

  let app = Application::builder().application_id("me.dejw-us").build();
  let (sender, receiver) = async_channel::bounded(1);

  app.connect_activate(move |app| {
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
    let listener = UnixListener::bind(socket_path).expect("Failed to bind to socket");
    for stream in listener.incoming() {
      let stream = stream.unwrap();
      sender
        .send_blocking(true)
        .expect("Failed to send blocking message");
    }
  });

  app.run();
}
