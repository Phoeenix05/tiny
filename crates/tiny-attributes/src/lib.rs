extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, Visibility};

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    
    // make sure that tiny's main function is private
    input.vis = Visibility::Inherited;

    // update tiny's main function name
    let tiny_main = Ident::new("__tiny_main", Span::call_site());
    input.sig.ident = tiny_main.clone();

    let output = quote! {
        #input

        pub fn main() -> () {
            #tiny_main();
        }
    };

    output.into()
}
