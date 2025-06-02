use crate::action::Action;
use crate::config::style::Style;
use crate::config::widget::Render;
use crate::context::Context;
use crate::peek::OptionPeek;
use crate::state::listener::StateListener;
use crate::state::StateValue;
use crate::text::Text;
use gtk4::prelude::{ButtonExt, Cast, ObjectExt, WidgetExt};
use gtk4::{Button, Widget};
use std::rc::Rc;

pub struct ButtonConfig {
  label: Text,
  on_click: Option<Action>,
  style: Option<Style>,
}

impl ButtonConfig {
  pub fn with_label(label: Text) -> Self {
    ButtonConfig {
      label,
      on_click: Some(Action::SetState("test".into(), StateValue::Int(100))),
      style: Some(Style::new()),
    }
  }
}

impl Render for ButtonConfig {
  fn render(&self, context: Rc<Context>) -> Widget {
    let button = Button::builder().build();
    let label = self.label.convert(
      context.as_ref(),
      || StateListener::Button(button.downgrade()),
      button.downgrade(),
    );
    button.set_label(&label);

    self.style.if_some(|style| {
      style.add_classes(&button);
      style.provider(button.display());
    });

    self.on_click.if_some(|on_click| {
      let on_click = on_click.clone();
      button.connect_clicked(move |_| {
        on_click.run(context.clone());
      });
    });

    button.upcast()
  }
}
