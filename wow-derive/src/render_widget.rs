use crate::utils::compile_error;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Type};

pub fn impl_render_widget(token: DeriveInput) -> TokenStream {
  let ident = &token.ident;
  let fields: Vec<_> = if let Data::Struct(data_struct) = token.data {
    if let Fields::Named(fields) = data_struct.fields {
      fields
        .named
        .iter()
        .map(|f| (f.clone().ident.unwrap(), f.ty.clone()))
        .collect()
    } else {
      return compile_error(ident, "Struct must contain only named fields");
    }
  } else {
    return compile_error(ident, "RenderWidget can only be used on structs");
  };

  let statements: Vec<_> = fields
    .iter()
    .map(|(ident, ty)| {
      if is_text_type(ty) {
        quote! {
          let label = self.#ident.convert(
            context.as_ref(),
            || Self::listener(widget.downgrade()),
            widget.downgrade(),
          );
          widget.set_text(&label);
        }
      } else if is_option_type(ty) {
        quote! {
          if let Some(a) = &self.#ident {
            a.apply(&widget, context.clone());
          }
        }
      } else {
        quote! {
          self.#ident.apply(&widget, context.clone());
        }
      }
    })
    .collect();

  quote! {
    impl crate::widget::RenderWidget for #ident {
      fn render(&self, context: std::rc::Rc<crate::context::Context>) -> gtk4::Widget {
        use gtk4::glib::clone::Downgrade;
        use crate::widget::ApplyWidget;
        use crate::display::TextDisplay;
        use gtk4::prelude::Cast;

        let widget = Self::build();
        #(#statements)*
        widget.upcast()
      }
    }
  }
  .into()
}

fn is_text_type(ty: &Type) -> bool {
  if let Type::Path(type_path) = ty {
    let last = type_path.path.segments.last().unwrap().ident.to_string();
    last == "Text"
  } else {
    false
  }
}

fn is_option_type(ty: &Type) -> bool {
  if let Type::Path(type_path) = ty {
    let segments = &type_path.path.segments;
    if segments.len() == 1 && segments[0].ident == "Option" {
      return true;
    }
  }
  false
}
