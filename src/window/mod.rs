use std::{collections::HashMap, fs::File, io::Read};

use gtk4::glib::object::Cast;
use gtk4::prelude::{BoxExt, StyleContextExt};
use gtk4::{
  Application, ApplicationWindow,
  gio::prelude::{ApplicationExt, ApplicationExtManual},
  prelude::{GtkWindowExt, WidgetExt},
};
use gtk4::{
  Box as GtkBox, CssProvider, Orientation, STYLE_PROVIDER_PRIORITY_APPLICATION, StyleContext,
};
use gtk4::{Label, Widget as GtkWidget};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Widget {
  Box(BoxWidget),
}

impl Widget {
  pub fn into_gtk(self) -> GtkWidget {
    match self {
      Widget::Box(box_widget) => box_widget.into_gtk(),
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BoxWidget {
  label: Option<String>,
  spacing: Option<i32>,
  orientation: Option<String>,
  #[serde(rename = "margin-all")]
  margin_all: Option<i32>,
}

impl BoxWidget {
  pub fn into_gtk(self) -> GtkWidget {
    let container = GtkBox::new(
      string_to_orientation(self.orientation.as_deref()),
      self.spacing.unwrap_or(0),
    );
    let label = Label::new(self.label.as_deref());
    if let Some(margin) = self.margin_all {
      println!("Margin: {}", margin);
      container.set_margin_bottom(margin);
      container.set_margin_end(margin);
      container.set_margin_start(margin);
      container.set_margin_top(margin);
    } else {
      println!("No margin");
    }
    container.append(&label);
    container.upcast()
  }
}

#[derive(Debug, Deserialize)]
pub struct Window {
  id: String,
  title: Option<String>,
  anchor: Option<Vec<String>>,
  height: Option<i32>,
  child: Widget,
}

fn string_to_orientation(string: Option<&str>) -> Orientation {
  if let Some(string) = string {
    match string.to_lowercase().as_str() {
      "vertical" => Orientation::Vertical,
      _ => Orientation::Horizontal,
    }
  } else {
    Orientation::Horizontal
  }
}

fn string_to_edge(string: &str) -> Edge {
  match string.to_lowercase().as_str() {
    "top" => Edge::Top,
    "right" => Edge::Right,
    "left" => Edge::Left,
    "bottom" => Edge::Bottom,
    _ => panic!("Failed to match"),
  }
}

impl Window {
  pub fn open(&self, css: &str) {
    let app = Application::builder().application_id(&self.id).build();
    let title = self.title.clone().unwrap();
    let anchor = self.anchor.clone().unwrap();
    let height = self.height.unwrap();
    let child = self.child.clone().into_gtk();
    let css = css.to_string();
    app.connect_activate(move |app| {
      let window = ApplicationWindow::builder()
        .application(app)
        .title(&title)
        .build();

      window.set_default_height(height);
      window.set_decorated(false);

      // layer shell
      window.init_layer_shell();
      window.set_layer(Layer::Top);
      window.auto_exclusive_zone_enable();

      // anchors
      for a in &anchor {
        window.set_anchor(string_to_edge(&a), true);
      }

      // css
      let provider = CssProvider::new();
      provider.load_from_data(&css);

      window
        .style_context()
        .add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
      window.add_css_class("hello");

      println!("Added css: {}", css);

      window.set_child(Some(&child));

      window.show();
    });

    app.run();
  }
}

#[derive(Debug)]
pub struct WindowManager {
  windows: HashMap<String, Window>,
}

impl WindowManager {
  pub fn from_config(config: &mut File) -> Self {
    let mut buf = String::new();
    config
      .read_to_string(&mut buf)
      .expect("Failed to read file");
    let windows: HashMap<String, Window> =
      serde_yaml::from_str(&buf).expect("Failed to parse yaml");
    Self { windows }
  }

  pub fn window(&self, window_name: &str) -> Option<&Window> {
    self.windows.get(window_name)
  }
}
