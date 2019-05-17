extern crate proc_macro;
use proc_macro::{TokenStream};

#[proc_macro_attribute]
pub fn powerset_enum(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
    // let input: ItemImpl = parse_macro_input!(input as ItemImpl);

    // match inherent_pub_impl(input) {
        // Ok(output) => {
            // output
        // },
        // Err(error) => {
            // error.to_compile_error().into()
        // }
    // }
}
