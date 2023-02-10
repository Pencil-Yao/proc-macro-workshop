use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let res_name = format_ident!("{name}Builder");

    // Parse fields in struct
    if let syn::Data::Struct(data) = input.data {}

    let output = quote!(
        impl #name {
            pub fn build() -> #res_name {
                #res_name {

                }
            }
        }
    );

    TokenStream::from(output)
}
