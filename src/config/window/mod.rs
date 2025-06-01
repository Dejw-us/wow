use crate::config::widget::Widget;
use crate::context::Context;
use gtk4::prelude::GtkWindowExt;
use gtk4::Application;

pub struct WindowConfig {
  child: Widget,
}

impl WindowConfig {
  pub fn with_child(child: Widget) -> WindowConfig {
    WindowConfig { child }
  }

  pub fn render(&self, app: &Application, context: &Context) {
    let window = gtk4::Window::builder()
      .application(app)
      .child(&self.child.render(context))
      .build();

    window.present();
  }
}
