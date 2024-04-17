use proc_macro2::Ident;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

use crate::trait_info::TraitInfo;

pub(crate) fn generate_reflect_trait(ast: &mut TraitInfo) -> syn::Result<proc_macro2::TokenStream> {
    let item_trait = &mut ast.item_trait;
    if !item_trait.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            item_trait,
            "Reflect trait should not have generic params.",
        ));
    }

    let span = item_trait.span();
    let syn::ItemTrait { supertraits, .. } = item_trait;
    {
        let mut segments = Punctuated::<syn::PathSegment, Token![::]>::new();
        segments.push(syn::PathSegment {
            ident: syn::Ident::new("Reflect", span),
            arguments: syn::PathArguments::None,
        });
        supertraits.push(syn::TypeParamBound::Trait(syn::TraitBound {
            path: syn::Path {
                leading_colon: None,
                segments,
            },
            paren_token: None,
            modifier: syn::TraitBoundModifier::None,
            lifetimes: None,
        }));
    }

    let trait_ident = &item_trait.ident;
    let mut name = "Reflect".to_string();
    name.push_str(&item_trait.ident.to_string());
    let reflect_ident = Ident::new(&name, item_trait.span());

    Ok(quote!(
        #item_trait

        pub struct #reflect_ident {
            pub get_func: fn(&dyn Reflect) -> &dyn #trait_ident,
            pub get_mut_func: fn(&mut dyn Reflect) -> &mut dyn #trait_ident,
            pub get_boxed_func: fn(Box<dyn Reflect>) -> Box<dyn #trait_ident>,
        }

        impl ReflectTrait for #reflect_ident {
            #[inline]
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl<T: Reflect + #trait_ident> FromType<T> for #reflect_ident {
            #[inline]
            fn from_type() -> Self {
                Self {
                    get_func: |obj| obj.as_any().downcast_ref::<T>().unwrap(),
                    get_mut_func: |obj| obj.as_any_mut().downcast_mut::<T>().unwrap(),
                    get_boxed_func: |obj| obj.as_any_boxed().downcast::<T>().unwrap(),
                }
            }
        }
    ))
}
