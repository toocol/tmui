use crate::{extend_element, extend_object, extend_widget};
use quote::quote;
use syn::{parse::Parser, DeriveInput, Ident};

pub(crate) fn expand(ast: &mut DeriveInput, id: Option<&String>) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;

    let mut run_after = false;

    for attr in ast.attrs.iter() {
        if let Some(attr_ident) = attr.path.get_ident() {
            if attr_ident.to_string() == "run_after" {
                run_after = true;
                break;
            }
        }
    }

    let run_after_clause = if run_after {
        quote!(
            ApplicationWindow::run_afters_of(self.window_id()).push(
                std::ptr::NonNull::new(self)
            );
        )
    } else {
        proc_macro2::TokenStream::new()
    };

    let set_shared_id_clause = match id {
        Some(id) => quote!(
            self.set_shared_id(#id);
        ),
        None => proc_macro2::TokenStream::new(),
    };

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub shared_widget: SharedWidget
                    })?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extends(SharedWidget)` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "shared_widget",
                vec!["shared_widget", "widget", "element", "object"],
                false,
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["shared_widget", "widget", "element"],
            )?;

            let widget_trait_impl_clause = extend_widget::gen_widget_trait_impl_clause(
                name,
                Some("shared_widget"),
                vec!["shared_widget", "widget"],
            )?;

            let shared_widget_trait_impl_clause =
                gen_shared_widget_trait_impl_clause(name, vec!["shared_widget"])?;

            Ok(quote! {
                #[derive(Derivative)]
                #[derivative(Default)]
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #shared_widget_trait_impl_clause

                impl WidgetAcquire for #name {}

                impl SuperType for #name {
                    #[inline]
                    fn super_type(&self) -> Type {
                        SharedWidget::static_type()
                    }
                }

                impl InnerInitializer for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                    }

                    #[inline]
                    fn inner_initialize(&mut self) {
                        #set_shared_id_clause
                        #run_after_clause
                    }
                }

                impl PointEffective for #name {
                    #[inline]
                    fn point_effective(&self, point: &Point) -> bool {
                        self.shared_widget.widget.point_effective(point)
                    }
                }

                impl ChildRegionAcquirer for #name {
                    #[inline]
                    fn child_region(&self) -> tlib::skia_safe::Region {
                        self.shared_widget.widget.child_region()
                    }
                }
            })
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends(SharedWidget)` has to be used with structs ",
        )),
    }
}

pub(crate) fn gen_shared_widget_trait_impl_clause(
    name: &Ident,
    shared_widget_path: Vec<&'static str>,
) -> syn::Result<proc_macro2::TokenStream> {
    let shared_widget_path: Vec<_> = shared_widget_path
        .iter()
        .map(|s| Ident::new(s, name.span()))
        .collect();

    Ok(quote!(
        impl SharedWidgetExt for #name {
            #[inline]
            fn shared_id(&self) -> &'static str {
                self.#(#shared_widget_path).*.shared_id()
            }

            #[inline]
            fn set_shared_id(&mut self, id: &'static str) {
                self.#(#shared_widget_path).*.set_shared_id(id)
            }
        }

        impl IsA<SharedWidget> for #name {}
    ))
}
