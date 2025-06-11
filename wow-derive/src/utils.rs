use proc_macro::TokenStream;
use syn::Ident;

pub fn compile_error(ident: &Ident, message: &str) -> TokenStream {
  syn::Error::new_spanned(ident, message)
    .into_compile_error()
    .into()
}
