use crate::extend_object;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub fn generate_extend_element(ast: &mut DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                pub element: Element
                            })?,
                    );
                }
                _ => (),
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "element",
                vec!["element", "object"],
            )?;

            let element_trait_impl_clause = gen_element_trait_impl_clause(name, vec!["element"])?;

            Ok(quote! {
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                impl ElementAcquire for #name {}
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends_element` has to be used with structs ",
        )),
    }
}

pub fn gen_element_trait_impl_clause(
    name: &Ident,
    element_path: Vec<&'static str>,
) -> syn::Result<proc_macro2::TokenStream> {
    let element_path: Vec<_> = element_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl ElementExt for #name {
            #[inline]
            fn update(&self) {
                self.set_property("invalidate", true.to_value());
            }

            #[inline]
            fn force_update(&self) {
                self.set_property("invalidate", true.to_value());
            }

            #[inline]
            fn rect(&self) -> Rect {
                self.#(#element_path).*.rect()
            }

            #[inline]
            fn set_fixed_width(&self, width: i32) {
                self.#(#element_path).*.set_fixed_width(width)
            }

            #[inline]
            fn set_fixed_height(&self, height: i32) {
                self.#(#element_path).*.set_fixed_height(height)
            }

            #[inline]
            fn set_fixed_x(&self, x: i32) {
                self.#(#element_path).*.set_fixed_x(x)
            }

            #[inline]
            fn set_fixed_y(&self, y: i32) {
                self.#(#element_path).*.set_fixed_y(y)
            }

            #[inline]
            fn invalidate(&self) -> bool {
                match self.get_property("invalidate") {
                    Some(invalidate) => invalidate.get::<bool>(),
                    None => false
                }
            }

            #[inline]
            fn validate(&self) {
                self.set_property("invalidate", false.to_value());
            }
        }

        impl IsA<Element> for #name {}
    ))
}
