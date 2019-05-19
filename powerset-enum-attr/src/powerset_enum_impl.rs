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
    let never_variant_trait_impls = gen_never_with_variant_trait_impl(&input.ident, &replaced_variants)?;
    let without_trait_impls = gen_without_trait_impls(&input.ident, &replaced_variants)?;
    let methods_on_enum_impl = gen_methods_on_enum_impl(&input.ident, &replaced_variants)?;
    let powerset_macro = gen_powerset_macro(&input.ident, &replaced_variants)?;

    Ok(quote!{
        #input
        #variant_trait_impls
        #error_from_trait_impls
        #never_variant_trait_impls
        #without_trait_impls
        #methods_on_enum_impl
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

fn make_never() -> syn::Type {
    syn::Type::Never(syn::TypeNever{
        bang_token: Default::default()
    })
}

fn gen_never_with_variant_trait_impl(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let impl_generics = make_generic_idents("T", 0..replaced_variants.len());
    let type_generics = make_generic_idents("T", 0..replaced_variants.len());
    Ok(quote!{
        impl<#(#impl_generics),*> powerset_enum::WithVariant<!> for #enum_ident<#(#type_generics),*> {
            type With = Self;
            fn add_possibility(self) -> Self::With {
                self
            }
        }
    })
}

fn gen_methods_on_enum_impl(
    enum_ident: &syn::Ident,
    replaced_variants: &[ReplacedVariant],
) -> Result<TokenStream, Error> {
    let first_source_generics = make_generic_idents("O", 0..replaced_variants.len());
    let first_source_generics = quote!(#(#first_source_generics),*);
    let final_target_generics = make_generic_idents("N", 0..replaced_variants.len());
    let final_target_generics = quote!(#(#final_target_generics),*);

    let where_bounds = replaced_variants.iter().map(|v| {
        let idx = v.idx;
        let source_generics = make_generic_idents("N", 0..idx).chain(make_generic_idents("O", idx..replaced_variants.len()));
        let target_generics = make_generic_idents("N", 0..idx+1).chain(make_generic_idents("O", idx+1..replaced_variants.len()));
        let current_type = make_generic_ident("N", idx);
        quote!{
            #enum_ident<#(#source_generics),*> : powerset_enum::WithVariant<#current_type, With=#enum_ident<#(#target_generics),*>>
        }
    });

    let add_possibility_statements = std::iter::repeat(quote!{
        let result = powerset_enum::WithVariant::add_possibility(result);
    }).take(replaced_variants.len());

    Ok(quote!{
        impl<#first_source_generics> #enum_ident<#first_source_generics> {
            pub fn upcast<#final_target_generics>(self) -> #enum_ident<#final_target_generics>
                where #(#where_bounds),*
            {
                let result = self;
                #(#add_possibility_statements)*
                result
            }
        }
    })
}

fn gen_with_variant_trait_impls(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let impls = replaced_variants.iter().map(|replaced_variant| {
        let ReplacedVariant {idx, ty, ..} = &replaced_variant;
        let impl_generics = replaced_variants.iter().filter(|v| v.idx != *idx).map(|v| make_generic_ident("T", v.idx));
        let impl_generics = quote!(#(#impl_generics),*);
        let source_generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                make_never()
            } else {
                make_generic_type(make_generic_ident("T", v.idx))
            }
        });
        let type_exists_source_generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                v.ty.clone()
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
        let add_possibility_match_arms = replaced_variants.iter().filter(|v| v.idx != *idx).map(|v| {
            let variant_ident = &v.variant_ident;
            quote!{
                #enum_ident::#variant_ident(value) => #enum_ident::#variant_ident(value)
            }
        });
        quote!{
            impl<#impl_generics> powerset_enum::WithVariant<#ty> for #enum_ident<#(#source_generic_params),*> {
                type With = #enum_ident<#(#target_generic_params),*>;
                fn add_possibility(self) -> Self::With {
                    match self {
                        #(#add_possibility_match_arms),*
                    }
                }
            }

            impl<#impl_generics> powerset_enum::WithVariant<#ty> for #enum_ident<#(#type_exists_source_generic_params),*> {
                type With = Self;
                fn add_possibility(self) -> Self::With {
                    self
                }
            }
        }
    });
    Ok(quote!(#( #impls )*))
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

fn gen_without_trait_impls(enum_ident: &syn::Ident, replaced_variants: &[ReplacedVariant]) -> Result<TokenStream, Error> {
    let impls = replaced_variants.iter().map(|replaced_variant| {
        let ReplacedVariant {idx, ty, ..} = &replaced_variant;
        let impl_generics = replaced_variants.iter().filter(|v| v.idx != *idx).map(|v| make_generic_ident("T", v.idx));
        let impl_generics = quote!(#(#impl_generics),*);
        let source_generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                ty.clone()
            } else {
                make_generic_type(make_generic_ident("T", v.idx))
            }
        });
        let target_generic_params = replaced_variants.iter().map(|v| {
            if v.idx == *idx {
                make_never()
            } else {
                make_generic_type(make_generic_ident("T", v.idx))
            }
        });
        let extract_match_arms = replaced_variants.iter().map(|v| {
            let variant_ident = &v.variant_ident;
            if v.idx == *idx {
                quote!{
                    #enum_ident::#variant_ident(value) => Err(value)
                }
            } else {
                quote!{
                    #enum_ident::#variant_ident(value) => Ok(#enum_ident::#variant_ident(value))
                }
            }
        });
        quote!{
            impl<#impl_generics> powerset_enum::WithoutVariant<#ty> for #enum_ident<#(#source_generic_params),*> {
                type Without = #enum_ident<#(#target_generic_params),*>;
                fn remove_possibility(self) -> Result<Self::Without, #ty> {
                    match self {
                        #(#extract_match_arms),*
                    }
                }
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
