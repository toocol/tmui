use crate::{
    extend_container,
    split_pane::{generate_split_pane_add_child, generate_split_pane_impl},
    stack::{generate_stack_add_child, generate_stack_impl}, scroll_area::{generate_scroll_area_add_child, generate_scroll_area_get_children, generate_scroll_area_impl},
};
use proc_macro2::Ident;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Meta};

const SUPPORTED_LAYOUTS: [&'static str; 5] = ["Stack", "VBox", "HBox", "SplitPane", "ScrollArea"];

pub(crate) fn expand(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
    internal: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    if SUPPORTED_LAYOUTS.contains(&layout) {
        gen_layout_clause(ast, layout, internal)
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

fn gen_layout_clause(ast: &mut DeriveInput, layout: &str, internal: bool) -> syn::Result<proc_macro2::TokenStream> {
    let has_content_alignment = layout == "VBox" || layout == "HBox";
    let has_size_unified_adjust = has_content_alignment;
    let is_vbox = layout == "VBox";
    let is_hbox = layout == "HBox";
    let is_split_pane = layout == "SplitPane";
    let is_stack = layout == "Stack";
    let is_scroll_area = layout == "ScrollArea";

    let mut token = extend_container::expand(
        ast,
        false,
        has_content_alignment,
        has_size_unified_adjust,
        is_split_pane,
        is_stack,
        is_scroll_area,
    )?;

    let name = &ast.ident;
    let span = ast.span();

    let children_fields = get_childrened_fields(ast);

    if is_split_pane && children_fields.len() > 0 {
        return Err(syn::Error::new_spanned(
            children_fields[0],
            "`SplitPane` can not use `#[children]` attribute, please use `add_child`, `split` functions instead.",
        ));
    }

    let layout = Ident::new(layout, span);

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

    let impl_size_unified_adjust = if is_vbox {
        quote!(
            impl SizeUnifiedAdjust for #name {
                #[inline]
                fn size_unified_adjust(&mut self) {
                    VBox::static_size_unified_adjust(self)
                }
            }
        )
    } else if is_hbox {
        quote!(
            impl SizeUnifiedAdjust for #name {
                #[inline]
                fn size_unified_adjust(&mut self) {
                    HBox::static_size_unified_adjust(self)
                }
            }
        )
    } else {
        proc_macro2::TokenStream::new()
    };

    let add_child_clause = match (is_split_pane, is_stack, is_scroll_area) {
        (false, false, false) => {
            quote! {
                use tmui::application_window::ApplicationWindow;
                child.set_parent(self);
                ApplicationWindow::initialize_dynamic_component(child.as_mut());
                self.container.children.push(child);
                self.update();
            }
        }
        (true, false, false) => generate_split_pane_add_child()?,
        (false, true, false) => generate_stack_add_child()?,
        (false, false, true) => generate_scroll_area_add_child(name)?,
        _ => unreachable!(),
    };

    let get_children_clause = if is_scroll_area {
        generate_scroll_area_get_children()?
    } else {
        quote!(
            fn children(&self) -> Vec<&dyn WidgetImpl> {
                let mut children: Vec<&dyn WidgetImpl> = self.container.children.iter().map(|c| c.as_ref()).collect();
                #(
                    children.push(self.#children_fields.as_ref());
                )*
                children
            }

            fn children_mut(&mut self) -> Vec<&mut dyn WidgetImpl> {
                let mut children: Vec<&mut dyn WidgetImpl> = self.container.children.iter_mut().map(|c| c.as_mut()).collect();
                #(
                    children.push(self.#children_fields.as_mut());
                )*
                children
            }
        )
    };

    let use_prefix = if internal { "crate" } else { "tmui" };

    let impl_split_pane = if is_split_pane {
        generate_split_pane_impl(name, use_prefix)?
    } else {
        proc_macro2::TokenStream::new()
    };

    let impl_stack_trait = if is_stack {
        generate_stack_impl(name, use_prefix)?
    } else {
        proc_macro2::TokenStream::new()
    };

    let impl_scroll_area = if is_scroll_area {
        generate_scroll_area_impl(name, use_prefix)?
    } else {
        proc_macro2::TokenStream::new()
    };

    token.extend(quote!(
        impl ContainerImpl for #name {
            #get_children_clause
        }

        impl ContainerImplExt for #name {
            fn add_child<T>(&mut self, mut child: Box<T>)
            where
                T: WidgetImpl,
            {
                #add_child_clause
            }
        }

        impl Layout for #name {
            fn composition(&self) -> Composition {
                #layout::static_composition(self)
            }

            fn position_layout(
                &mut self,
                previous: Option<&dyn WidgetImpl>,
                parent: Option<&dyn WidgetImpl>,
                manage_by_container: bool,
            ) {
                #layout::container_position_layout(self, previous, parent, manage_by_container)
            }
        }

        impl ObjectChildrenConstruct for #name {
            fn children_construct(&mut self) {
                #(
                    if !self.#children_fields.constructed() {
                        self.#children_fields.construct();
                    }
                    let self_ptr = self as *mut dyn WidgetImpl;
                    self.#children_fields.set_parent(self_ptr);
                )*
            }
        }

        impl ContainerScaleCalculate for #name {
            #[inline]
            fn container_hscale_calculate(&self) -> f32 {
                #layout::static_container_hscale_calculate(self)
            }

            #[inline]
            fn container_vscale_calculate(&self) -> f32 {
                #layout::static_container_vscale_calculate(self)
            }
        }

        #impl_content_alignment

        #impl_size_unified_adjust

        #impl_split_pane

        #impl_stack_trait

        #impl_scroll_area

        impl ChildContainerDiffRender for #name {
            fn container_diff_render(&mut self, painter: &mut Painter, background: Color) {
            }
        }
    ));

    Ok(token)
}
