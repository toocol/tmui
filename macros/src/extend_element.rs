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
                        "`extends(Element)` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "element",
                vec!["element", "object"],
                false,
            )?;

            let element_trait_impl_clause = gen_element_trait_impl_clause(name, vec!["element"])?;

            Ok(quote! {
                #[derive(Derivative)]
                #[derivative(Default)]
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                impl ElementAcquire for #name {}

                impl SuperType for #name {
                    #[inline]
                    fn super_type(&self) -> Type {
                        Element::static_type()
                    }
                }

                impl InnerInitializer for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectElementImpl>();
                    }
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(Element)` has to be used with structs ",
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
                Board::notify_update()
            }

            #[inline]
            fn force_update(&mut self) {
                self.update();
                Board::force_update();
            }

            #[inline]
            fn update_rect(&mut self, rect: Rect) {
                self.set_property("invalidate", true.to_value());
                Board::notify_update();
                self.#(#element_path).*.update_rect(rect);
            }

            #[inline]
            fn update_rect_f(&mut self, rect: FRect) {
                self.set_property("invalidate", true.to_value());
                Board::notify_update();
                self.#(#element_path).*.update_rect_f(rect);
            }

            #[inline]
            fn update_region(&mut self, region: &Region) {
                self.set_property("invalidate", true.to_value());
                Board::notify_update();
                self.#(#element_path).*.update_region(region);
            }

            #[inline]
            fn update_region_f(&mut self, region: &FRegion) {
                self.set_property("invalidate", true.to_value());
                Board::notify_update();
                self.#(#element_path).*.update_region_f(region);
            }

            #[inline]
            fn clear_region(&mut self) {
                self.#(#element_path).*.clear_region();
            }

            #[inline]
            fn clear_region_f(&mut self) {
                self.#(#element_path).*.clear_region_f();
            }

            #[inline]
            fn redraw_region(&self) -> &Region {
                self.#(#element_path).*.redraw_region()
            }

            #[inline]
            fn redraw_region_f(&self) -> &FRegion {
                self.#(#element_path).*.redraw_region_f()
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
                self.#(#element_path).*.invalidate()
            }

            #[inline]
            fn validate(&mut self) {
                self.#(#element_path).*.validate()
            }

            #[inline]
            fn rect_record(&self) -> Rect {
                self.#(#element_path).*.rect_record()
            }

            #[inline]
            fn set_rect_record(&mut self, rect: Rect) {
                self.#(#element_path).*.set_rect_record(rect)
            }
        }

        impl IsA<Element> for #name {}
    ))
}
