use crate::{extend_container, split_pane::generate_split_pane_impl};
use proc_macro2::Ident;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Meta};

const SUPPORTED_LAYOUTS: [&'static str; 4] = ["Stack", "VBox", "HBox", "SplitPane"];

pub(crate) fn expand(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    if SUPPORTED_LAYOUTS.contains(&layout) {
        gen_layout_clause(ast, layout)
    } else {
        Err(syn::Error::new_spanned(
            layout_meta,
            format!("Invalid layout: {}", layout),
        ))
    }
}

/// Get the fileds' Ident which defined the attribute `#[children]` provided by the derive macro [`Childrenable`](crate::Childrenable)
fn get_childrened_fields<'a>(ast: &'a DeriveInput) -> Vec<&'a Ident> {
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

fn gen_layout_clause(ast: &mut DeriveInput, layout: &str) -> syn::Result<proc_macro2::TokenStream> {
    let has_content_alignment = layout == "VBox" || layout == "HBox";
    let is_split_pane = layout == "SplitPane";
    let mut token = extend_container::expand(ast, false, has_content_alignment, is_split_pane)?;
    let name = &ast.ident;

    let children_fields = get_childrened_fields(ast);

    let layout = Ident::new(layout, ast.span());

    let impl_content_alignment = if has_content_alignment {
        quote!(
            impl ContentAlignment for #name {
                #[inline]
                fn homogeneous(&self) -> bool {
                    self.homogeneous
                }

                #[inline]
                fn set_homogeneous(&mut self, homogeneous: bool) {
                    self.homogeneous = homogeneous
                }

                #[inline]
                fn content_halign(&self) -> Align {
                    self.content_halign
                }

                #[inline]
                fn content_valign(&self) -> Align {
                    self.content_valign
                }

                #[inline]
                fn set_content_halign(&mut self, halign: Align) {
                    self.content_halign = halign
                }

                #[inline]
                fn set_content_valign(&mut self, valign: Align) {
                    self.content_valign = valign
                }
            }
        )
    } else {
        proc_macro2::TokenStream::new()
    };

    let add_child_clause = if is_split_pane {
        quote! {
            if self.children.len() != 0 {
                panic!("Only first widget can use function `add_child()` to add, please use `split_left()`,`split_top()`,`split_right()` or `split_down()`")
            }
            let mut child = Box::new(child);
            let widget_ptr: std::option::Option<std::ptr::NonNull<dyn WidgetImpl>> = std::ptr::NonNull::new(child.as_mut());
            let mut split_info = Box::new(SplitInfo::new(
                child.id(),
                widget_ptr.clone(),
                None,
                SplitType::SplitNone,
            ));
            self.split_infos_vec
                .push(std::ptr::NonNull::new(split_info.as_mut()));
            self.split_infos.insert(child.id(), split_info);
            self.children.push(child);
        }
    } else {
        quote! {
            self.children.push(Box::new(child))
        }
    };

    let impl_split_pane = if is_split_pane {
        generate_split_pane_impl(name)?
    } else {
        proc_macro2::TokenStream::new()
    };

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
                #add_child_clause
            }
        }

        impl Layout for #name {
            fn composition(&self) -> Composition {
                #layout::static_composition()
            }

            fn position_layout(
                &mut self,
                previous: &dyn WidgetImpl,
                parent: &dyn WidgetImpl,
                manage_by_container: bool,
            ) {
                #layout::container_position_layout(self, previous, parent, manage_by_container)
            }
        }

        impl ObjectChildrenConstruct for #name {
            fn children_construct(&mut self) {
                #(
                    self.#children_fields.construct();
                )*
            }
        }

        #impl_content_alignment

        #impl_split_pane
    ));

    Ok(token)
}
