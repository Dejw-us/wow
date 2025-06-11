use crate::config::ApplyWidget;
use crate::peek::OptionPeek;
use crate::util::file::read_file_to_string;
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

impl ApplyWidget for Style {
  fn apply(&self, widget: &impl WidgetExt) {
    let display = widget.display();
    let provider = CssProvider::new();
    self.file.if_some(|css| {
      provider.load_from_data(&read_file_to_string(css).expect("Failed to read css"));
      style_context_add_provider_for_display(
        &display,
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
      );
    });
    for class in self.classes.iter() {
      widget.add_css_class(class);
    }
  }
}
