use crate::render_widget::impl_render_widget;
use syn::parse_macro_input;

mod render_widget;
mod utils;

#[proc_macro_derive(RenderWidget)]
pub fn render(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  impl_render_widget(parse_macro_input!(input as syn::DeriveInput))
}
