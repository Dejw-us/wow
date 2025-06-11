use crate::config::ApplyWidget;
use crate::peek::OptionPeek;
use gtk4::prelude::WidgetExt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Align {
  horizontal: Option<AlignType>,
  vertical: Option<AlignType>,
}

impl ApplyWidget for Align {
  fn apply(&self, widget: &impl WidgetExt) {
    self
      .horizontal
      .if_some(|align| widget.set_halign(align.into()));
    self
      .vertical
      .if_some(|align| widget.set_valign(align.into()));
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlignType {
  Center,
  End,
  Fill,
  Start,
  Baseline,
}

impl Into<gtk4::Align> for &AlignType {
  fn into(self) -> gtk4::Align {
    match self {
      AlignType::Center => gtk4::Align::Center,
      AlignType::End => gtk4::Align::End,
      AlignType::Fill => gtk4::Align::Fill,
      AlignType::Start => gtk4::Align::Start,
      AlignType::Baseline => gtk4::Align::Baseline,
    }
  }
}
