use crate::extend_container;
use proc_macro2::Ident;
use quote::quote;
use syn::{DeriveInput, Meta};

pub(crate) fn expand(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    match layout {
        "Stack" => {
            let mut token = extend_container::expand(ast)?;
            let name = &ast.ident;

            let children_fields = get_children_fields(ast);

            token.extend(quote!(
                impl ContainerImpl for #name {
                    fn children(&self) -> Vec<&dyn WidgetImpl> {
                        let mut children: Vec<&dyn WidgetImpl> = self.children.iter().map(|c| c.as_ref()).collect();
                        #(
                            children.push(&self.#children_fields);
                        )*
                        children
                    }

                    fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
                        let mut children: Vec<&mut dyn WidgetImpl> = self.children.iter_mut().map(|c| c.as_mut()).collect();
                        #(
                            children.push(&mut self.#children_fields);
                        )*
                        children
                    }
                }

                impl ContainerImplExt for #name {
                    fn add_child<T>(&mut self, child: T)
                    where
                        T: WidgetImpl + IsA<Widget>,
                    {
                        self.children.push(Box::new(child))
                    }
                }

                impl Layout for #name {
                    fn composition(&self) -> Composition {
                        Composition::Overlay
                    }

                    fn position_layout(
                        &mut self,
                        previous: &dyn WidgetImpl,
                        parent: &dyn WidgetImpl,
                        manage_by_container: bool,
                    ) {
                        Stack::container_position_layout(self, previous, parent, manage_by_container)
                    }
                }
            ));

            Ok(token)
        }
        "VBox" => {
            todo!()
        }
        "HBox" => {
            todo!()
        }
        _ => {
            return Err(syn::Error::new_spanned(
                layout_meta,
                format!("Invalid layout: {}", layout),
            ))
        }
    }
}

fn get_children_fields<'a>(ast: &'a DeriveInput) -> Vec<&'a Ident> {
    let mut children_idents = vec![];
    match &ast.data {
        syn::Data::Struct(ref struct_data) => match &struct_data.fields {
            syn::Fields::Named(syn::FieldsNamed { ref named, .. }) => {
                for field in named.iter() {
                    for attr in field.attrs.iter() {
                        if let Some(attr_ident) = attr.path.get_ident() {
                            if attr_ident.to_string() == "children" {
                                children_idents.push(field.ident.as_ref().unwrap());
                                break;
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    children_idents
}
