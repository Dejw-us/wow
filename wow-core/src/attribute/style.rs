use crate::context::Context;
use crate::widget::ApplyWidget;
use gtk4::prelude::WidgetExt;
use gtk4::{
  style_context_add_provider_for_display, CssProvider, Widget, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use serde::Deserialize;
use std::rc::Rc;
use wow_utils::option::IfSome;
use wow_utils::read_file_to_string;

#[derive(Deserialize, Debug)]
pub struct Style {
  file: Option<String>,
  classes: Option<Vec<String>>,
}

impl ApplyWidget for Style {
  fn apply(&self, widget: &Widget, context: Rc<Context>) {
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
    if let Some(classes) = self.classes.as_ref() {
      for class in classes.iter() {
        widget.add_css_class(class);
      }
    }
  }
}
