use crate::config::style::Style;
use crate::config::widget::{Render, Widget};
use crate::config::window::anchor::WindowAnchor;
use crate::config::window::layer::WindowLayer;
use crate::context::Context;
use crate::peek::OptionPeek;
use gtk4::prelude::{Cast, GtkWindowExt, WidgetExt};
use gtk4::Application;
use gtk4_layer_shell::LayerShell;
use std::rc::Rc;

pub mod anchor;
pub mod layer;

pub struct WindowConfig {
  child: Widget,
  anchor: Vec<WindowAnchor>,
  layer: Option<WindowLayer>,
  classes: Vec<String>,
  style: Option<Style>,
}

impl WindowConfig {
  pub fn with_child(child: Widget) -> WindowConfig {
    WindowConfig {
      child,
      anchor: vec![WindowAnchor::Top, WindowAnchor::Left, WindowAnchor::Right],
      layer: Some(WindowLayer::Top),
      classes: vec!["test".to_string()],
      style: Some(Style::new()),
    }
  }

  pub fn render(&self, app: &Application, context: Rc<Context>) {
    let window = gtk4::Window::builder()
      .application(app)
      .child(&self.child.render(context))
      .build();

    self.style.if_some(|style| {
      for class in style.classes() {
        window.add_css_class(class);
      }
      style.provider(window.display());
    });

    self.layer.if_some(|layer| {
      window.init_layer_shell();
      window.set_layer(layer.clone().into());
    });

    for anchor in &self.anchor {
      window.set_anchor(anchor.clone().into(), true);
    }

    window.auto_exclusive_zone_enable();

    window.present();
  }
}
