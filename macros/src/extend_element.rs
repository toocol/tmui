use crate::extend_object;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub(crate) fn expand(ast: &mut DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub element: Element
                    })?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extend_element` should defined on named fields struct.",
                    ))
                }
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

                impl ParentType for #name {
                    #[inline]
                    fn parent_type(&self) -> Type {
                        Element::static_type()
                    }
                }

                impl InnerTypeRegister for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectElementImpl>();
                    }
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends_element` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_element_trait_impl_clause(
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
            fn set_window_id(&mut self, id: u16) {
                self.#(#element_path).*.set_window_id(id)
            }

            #[inline]
            fn window_id(&self) -> u16 {
                self.#(#element_path).*.window_id()
            }

            #[inline]
            fn update(&mut self) {
                self.set_property("invalidate", true.to_value());
            }

            #[inline]
            fn force_update(&mut self) {
                self.set_property("invalidate", true.to_value());
            }

            #[inline]
            fn rect(&self) -> Rect {
                self.#(#element_path).*.rect()
            }

            #[inline]
            fn set_fixed_width(&mut self, width: i32) {
                self.#(#element_path).*.set_fixed_width(width)
            }

            #[inline]
            fn set_fixed_height(&mut self, height: i32) {
                self.#(#element_path).*.set_fixed_height(height)
            }

            #[inline]
            fn set_fixed_x(&mut self, x: i32) {
                self.#(#element_path).*.set_fixed_x(x)
            }

            #[inline]
            fn set_fixed_y(&mut self, y: i32) {
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
            fn validate(&mut self) {
                self.set_property("invalidate", false.to_value());
            }
        }

        impl IsA<Element> for #name {}
    ))
}
