use crate::{extend_element, extend_object, extend_widget};
use quote::quote;
use syn::{parse::Parser, DeriveInput};

pub(crate) fn expand(ast: &mut DeriveInput, impl_children_construct: bool) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub container: Container
                    })?);
                    fields.named.push(syn::Field::parse_named.parse2(quote! {
                        pub children: Vec<Box<dyn WidgetImpl>>
                    })?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "`extend_container` should defined on named fields struct.",
                    ))
                }
            }

            let object_trait_impl_clause = extend_object::gen_object_trait_impl_clause(
                name,
                "container",
                vec!["container", "widget", "element", "object"],
                true
            )?;

            let element_trait_impl_clause = extend_element::gen_element_trait_impl_clause(
                name,
                vec!["container", "widget", "element"],
            )?;

            let widget_trait_impl_clause =
                extend_widget::gen_widget_trait_impl_clause(name, vec!["container", "widget"])?;

            let mut children_construct_clause = proc_macro2::TokenStream::new();
            if impl_children_construct {
                children_construct_clause.extend(quote!(
                    impl ObjectChildrenConstruct for #name {}
                ))
            }

            Ok(quote!(
                #ast

                #object_trait_impl_clause

                #element_trait_impl_clause

                #widget_trait_impl_clause

                #children_construct_clause

                impl ContainerAcquire for #name {}

                impl ParentType for #name {
                    #[inline]
                    fn parent_type(&self) -> Type {
                        Container::static_type()
                    }
                }

                impl InnerTypeRegister for #name {
                    #[inline]
                    fn inner_type_register(&self, type_registry: &mut TypeRegistry) {
                        type_registry.register::<#name, ReflectWidgetImpl>();
                        type_registry.register::<#name, ReflectContainerImpl>();
                        type_registry.register::<#name, ReflectObjectChildrenConstruct>();
                    }
                }
            ))
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "`extends_container` has to be used with structs ",
        )),
    }
}
