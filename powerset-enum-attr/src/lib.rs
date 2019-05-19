#![recursion_limit="128"]
extern crate proc_macro;

use syn::parse_macro_input;

mod powerset_enum_impl;
mod powerset_macro_impl;

#[proc_macro]
pub fn powerset(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match powerset_macro_impl::powerset_macro_impl(parse_macro_input!(args)) {
        Ok(output) => {
            output.into()
        },
        Err(error) => {
            error.to_compile_error().into()
        }
    }
}

#[proc_macro_attribute]
pub fn powerset_enum(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match powerset_enum_impl::powerset_enum_impl(parse_macro_input!(input)) {
        Ok(output) => {
            output.into()
        },
        Err(error) => {
            error.to_compile_error().into()
        }
    }
}

