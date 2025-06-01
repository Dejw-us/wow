use crate::config::text::Text;
use crate::config::widget::Widget;
use crate::context::Context;
use gtk4::prelude::Cast;
use gtk4::Label;

pub struct LabelConfig {
  label: Text,
}

impl Into<Widget> for LabelConfig {
  fn into(self) -> Widget {
    Widget::Label(self)
  }
}

impl LabelConfig {
  pub fn with_label(label: Text) -> Self {
    LabelConfig { label }
  }

  pub fn render(&self, context: &Context) -> gtk4::Widget {
    let label = match &self.label {
      Text::Text(text) => text.into(),
      Text::State(state_name) => {
        if let Some(state) = context.get_state(&state_name) {
          state.get().to_string()
        } else {
          String::new()
        }
      }
    };
    let label = Label::builder().label(&label).build();
    label.upcast()
  }
}
