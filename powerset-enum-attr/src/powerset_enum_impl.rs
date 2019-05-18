use proc_macro2::{TokenStream};

use syn::parse::Error;
use quote::quote;

pub fn powerset_enum_impl(mut input: syn::ItemEnum) -> Result<TokenStream, Error> {
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(input.generics, "powerset-enum does not support generics"));
    }

    let mut replaced_variants = Vec::new();

    for (idx, variant) in input.variants.iter_mut().enumerate() {
        if variant.discriminant.is_some() {
            return Err(Error::new_spanned(variant, "powerset-enum variants cannot have discriminants"));
        }
        let field = match &mut variant.fields {
            syn::Fields::Named(_) => None,
            syn::Fields::Unit => None,
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    fields.unnamed.iter_mut().next()
                } else {
                    None
                }
            }
        };
        let field = if let Some(field) = field {
            field
        } else {
            return Err(Error::new_spanned(variant, "powerset-enum variants must contain a single unnamed item"));
        };

        let generic_ident = make_generic_ident("T", idx);
        let original_type = std::mem::replace(&mut field.ty, make_generic_type(generic_ident.clone()));

        replaced_variants.push(ReplacedVariant {
            idx,
            ty: original_type,
            variant_ident: variant.ident.clone(),
        });

        input.generics.params.push(syn::GenericParam::Type(generic_ident.into()));
    }

    let variant_trait_impls = gen_with_variant_trait_impls(&input.ident, &replaced_variants)?;
    let error_from_trait_impls = gen_error_from_trait_impls(&input.ident, &replaced_variants)?;
    let enum_from_trait_impls = gen_enum_from_trait_impls(&input.ident, &replaced_variants)?;
    // println!("{}", enum_from_trait_impls);
    let powerset_macro = gen_powerset_macro(&input.ident, &replaced_variants)?;

    Ok(quote!{
        #input
        #variant_trait_impls
        #error_from_trait_impls
        // #enum_from_trait_impls
        #powerset_macro
    })
}

#[derive(Debug)]
struct ReplacedVariant {
    idx: usize,
    ty: syn::Type,
    variant_ident: syn::Ident,
}

fn make_generic_ident(prefix: &str, idx: usize) -> syn::Ident {
    syn::Ident::new(&format!("{}{}", prefix, idx), proc_macro2::Span::call_site())
}

fn make_generic_idents(prefix: &'static str, rng: std::ops::Range<usize>) -> impl Iterator<Item = syn::Ident> {
    rng.map(move |i| make_generic_ident(prefix, i))
}

fn make_generic_type(ident: syn::Ident) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: ident.into(),
    })
}

fn make_generic_types(prefix: &'static str, rng: std::ops::Range<usize>) -> impl Iterator<Item = syn::Type> {
    rng.map(move |i| make_generic_type(make_generic_ident(prefix, i)))
}

fn make_never() -> syn::Type {
    syn::Type::Never(syn::TypeNever{
        bang_token: Default::default()
    })
}

fn gen_error_from_trait_impls(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let impls = replaced_variants.iter().map(|replaced_variant| {
        let ReplacedVariant {idx, ty, variant_ident} = &replaced_variant;
        let impl_generics = replaced_variants.iter().filter(|v| v.idx != *idx).map(|v| make_generic_ident("T", v.idx));
        let generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                ty.clone()
            } else {
                make_generic_type(make_generic_ident("T", v.idx))
            }
        });
        quote!{
            impl<#(#impl_generics),*> From<#ty> for #enum_ident<#(#generic_params),*> {
                fn from(value: #ty) -> Self {
                    #enum_ident::#variant_ident(value)
                }
            }
        }
    });
    Ok(quote!(#( #impls )*))
}

fn gen_enum_from_trait_impls(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let impls = replaced_variants.iter().map(|replaced_variant| {
        let ReplacedVariant {idx, ty, ..} = &replaced_variant;
        let impl_generics = make_generic_idents("O", 0..*idx)
            .chain(make_generic_idents("N", 0..*idx))
            .chain(make_generic_idents("T", (*idx + 1)..replaced_variants.len()));
        let from_generic_params = make_generic_types("O", 0..*idx)
            .chain(std::iter::once(make_never()))
            .chain(make_generic_types("T", (*idx + 1)..replaced_variants.len()));
        let from_type = quote!(#enum_ident<#(#from_generic_params),*>);
        let to_generic_params = make_generic_types("N", 0..*idx)
            .chain(std::iter::once(ty.clone()))
            .chain(make_generic_types("T", (*idx + 1)..replaced_variants.len()));
        let mid_generic_params = make_generic_types("N", 0..*idx)
            .chain(std::iter::once(make_never()))
            .chain(make_generic_types("T", (*idx + 1)..replaced_variants.len()));
        let mid_type = quote!(#enum_ident<#(#mid_generic_params),*>);
        let variant_match_arms = replaced_variants.iter().filter(|v| v.idx != *idx).map(|other_variant| {
            let variant_ident = &other_variant.variant_ident;
            quote!{
                #enum_ident::#variant_ident(e) => #enum_ident::#variant_ident(e)
            }
        });
        quote!{
            impl<#(#impl_generics),*> From<#from_type> for #enum_ident<#(#to_generic_params),*>
                where #mid_type: From<#from_type>
            {
                fn from(value: #from_type) -> Self {
                    let mid_value: #mid_type = #enum_ident::from(value);
                    match mid_value {
                        #(#variant_match_arms),*
                    }
                }
            }
        }
    });
    Ok(quote!(#( #impls )*))
}

fn gen_with_variant_trait_impls(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let impls = replaced_variants.iter().map(|replaced_variant| {
        let ReplacedVariant {idx, ty, ..} = &replaced_variant;
        let impl_generics = replaced_variants.iter().filter(|v| v.idx != *idx).map(|v| make_generic_ident("T", v.idx));
        let source_generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                make_never()
            } else {
                make_generic_type(make_generic_ident("T", v.idx))
            }
        });
        let target_generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                ty.clone()
            } else {
                make_generic_type(make_generic_ident("T", v.idx))
            }
        });
        quote!{
            impl<#(#impl_generics),*> powerset_enum::WithVariant<#ty> for #enum_ident<#(#source_generic_params),*> {
                type With = #enum_ident<#(#target_generic_params),*>;
            }
        }
    });
    Ok(quote!(#( #impls )*))
}

fn gen_powerset_macro(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let empty_powerset_generics = replaced_variants.iter().map(|_| make_never());
    let empty_powerset = quote!(#enum_ident<#(#empty_powerset_generics),*>);
    Ok(quote!{
        macro_rules! #enum_ident {
            ($($tt:ty),*) => { powerset_enum::powerset!(#empty_powerset, $($tt),*) };
            ($($tt:ty),*,) => { powerset_enum::powerset!(#empty_powerset, $($tt),*) };
        }
    })
}
