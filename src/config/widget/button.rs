use crate::action::Action;
use crate::config::geometry::Geometry;
use crate::config::style::Style;
use crate::config::widget::Render;
use crate::config::{align, ApplyWidget};
use crate::context::Context;
use crate::peek::OptionPeek;
use crate::state::listener::StateListener;
use crate::text::Text;
use gtk4::prelude::{ButtonExt, Cast, ObjectExt};
use gtk4::{Button, Widget};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug)]
pub struct ButtonConfig {
  label: Text,
  #[serde(rename = "on-click")]
  on_click: Option<Action>,
  style: Option<Style>,
  geometry: Option<Geometry>,
  align: Option<align::Align>,
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

    self.align.if_some(|align| align.apply(&button));

    self.geometry.if_some(|g| g.apply(&button));

    self.style.if_some(|style| style.apply(&button));

    self.on_click.if_some(|on_click| {
      let on_click = on_click.clone();
      button.connect_clicked(move |_| {
        on_click.run(context.clone());
      });
    });

    button.upcast()
  }
}
