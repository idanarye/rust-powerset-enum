use proc_macro2::{TokenStream};

use syn::parse::Error;
use quote::quote;

#[derive(Debug)]
pub struct PowersetMacroInput {
    empty_powerset: syn::Type,
    comma: Option<syn::token::Comma>,
    types_in_powerset: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
}

impl syn::parse::Parse for PowersetMacroInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, Error> {
        Ok(PowersetMacroInput {
            empty_powerset: input.parse()?,
            comma: input.parse()?,
            types_in_powerset: syn::punctuated::Punctuated::parse_terminated(input)?,
        })
    }
}

pub fn powerset_macro_impl(input: PowersetMacroInput) -> Result<TokenStream, Error> {
    let PowersetMacroInput { empty_powerset, types_in_powerset, .. } = input;
    let mut result = quote!(#empty_powerset);
    for ty in types_in_powerset {
        result = quote!(<#result as powerset_enum::WithVariant<#ty>>::With);
    }
    Ok(result)
}
