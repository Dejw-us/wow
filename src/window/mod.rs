use std::env::set_var;

use derive_builder::Builder;
use gtk4::{
  Application, ApplicationWindow,
  gio::prelude::{ApplicationExt, ApplicationExtManual},
  prelude::{GtkWindowExt, WidgetExt},
};
use serde::Deserialize;

use crate::widget::{Widget, WidgetRender};

#[derive(Builder, Deserialize)]
pub struct Window {
  id: String,
  child: Option<Widget>,
}

impl Window {
  pub fn new(id: String) -> Self {
    Self { child: None, id }
  }

  pub fn open(&self, force_wayland: bool) {
    if force_wayland {
      unsafe {
        set_var("GDK_BACKEND", "wayland");
      }
    }

    let app = Application::builder().application_id(&self.id).build();
    let child = self.child.clone();

    app.connect_activate(move |app| {
      let window = ApplicationWindow::builder().application(app).build();

      window.set_child(child.as_ref().map(|c| c.render()).as_ref());

      window.show();
    });

    app.run();
  }
}
