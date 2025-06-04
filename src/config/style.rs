use crate::peek::OptionPeek;
use crate::util::file::read_file_to_string;
use gtk4::gdk::Display;
use gtk4::prelude::WidgetExt;
use gtk4::{
  style_context_add_provider_for_display, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Style {
  file: Option<String>,
  classes: Vec<String>,
}

impl Style {
  pub fn new() -> Self {
    Self {
      file: Some("style.css".to_string()),
      classes: vec!["test".to_string()],
    }
  }

  pub fn add_classes(&self, widget: &impl WidgetExt) {
    for class in self.classes.iter() {
      widget.add_css_class(class);
    }
  }

  pub fn provider(&self, display: Display) {
    let provider = CssProvider::new();
    self.file.if_some(|css| {
      provider.load_from_data(&read_file_to_string(css).expect("Failed to read css"));
      style_context_add_provider_for_display(
        &display,
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
      );
    });
  }
}
