use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use hypr_ipc::error::Result;

pub mod window;

fn main() -> Result<()> {
  unsafe {
    std::env::set_var("GDK_BACKEND", "wayland");
  }

  let app = Application::builder().application_id("com.example").build();
  gtk4::init().expect("Failed to init");
  println!("Supported: {}", gtk4_layer_shell::is_supported());
  app.connect_activate(|app| {
    let window = ApplicationWindow::builder()
      .application(app)
      .title("Panel")
      .default_width(1920)
      .default_height(30)
      .build();

    window.set_decorated(false);

    let container = GtkBox::new(Orientation::Horizontal, 0);
    container.append(&Label::new(Some("This is a fixed panel")));

    window.set_child(Some(&container));

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.auto_exclusive_zone_enable();
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Left, true);
    window.set_exclusive_zone(30);

    window.show();
  });

  app.run();
  Ok(())
}
