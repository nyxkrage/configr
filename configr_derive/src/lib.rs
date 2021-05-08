use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Configr)]
pub fn configr(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    format!("impl Config<Self> for {} {{}}", ident).parse().unwrap()
}