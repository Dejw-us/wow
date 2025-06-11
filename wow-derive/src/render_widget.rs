use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_render_widget(token: DeriveInput) -> TokenStream {
  let ident = &token.ident;

  quote! {
    impl wow_common::widget::RenderWidget for #ident {
      fn render(&self, context: std::rc::Rc<wow_common::context::Context>) -> gtk4::Widget {
        let button = gtk4::Button::builder().build();
        println!("Creating button");
        use gtk4::prelude::Cast;
        button.upcast()
      }
    }
  }
  .into()
}
