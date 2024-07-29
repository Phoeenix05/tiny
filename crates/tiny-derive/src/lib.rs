extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let name = &input.sig.ident.to_string();

    if name != &"tiny".to_owned() {
        panic!(
            "this attribute should only be used on fn named `tiny`, found `{}`.",
            name
        );
    }

    let output = quote! {
        #input

        pub fn main() -> () {}
    };

    output.into()
}
