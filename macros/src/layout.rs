use crate::{
    extend_container,
    pane::{generate_pane_add_child, generate_pane_impl},
    scroll_area::{
        generate_scroll_area_add_child, generate_scroll_area_get_children,
        generate_scroll_area_impl,
    },
    split_pane::{generate_split_pane_add_child, generate_split_pane_impl},
    stack::{generate_stack_add_child, generate_stack_impl},
};
use proc_macro2::Ident;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Meta};

const STR_STACK: &str = "Stack";
const STR_VBOX: &str = "VBox";
const STR_HBOX: &str = "HBox";
const STR_SPLIT_PANE: &str = "SplitPane";
const STR_SCROLL_AREA: &str = "ScrollArea";
const STR_PANE: &str = "Pane";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LayoutType {
    Non,
    Stack,
    VBox,
    HBox,
    SplitPane,
    ScrollArea,
    Pane,
}
impl LayoutType {
    pub fn from(meta: &Meta, value: &str) -> syn::Result<Self> {
        match value {
            STR_STACK => Ok(Self::Stack),
            STR_VBOX => Ok(Self::VBox),
            STR_HBOX => Ok(Self::HBox),
            STR_SPLIT_PANE => Ok(Self::SplitPane),
            STR_SCROLL_AREA => Ok(Self::ScrollArea),
            STR_PANE => Ok(Self::Pane),
            _ => Err(syn::Error::new_spanned(
                meta,
                format!("Unsupported layout type {}.", value),
            )),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stack => STR_STACK,
            Self::VBox => STR_VBOX,
            Self::HBox => STR_HBOX,
            Self::SplitPane => STR_SPLIT_PANE,
            Self::ScrollArea => STR_SCROLL_AREA,
            Self::Pane => STR_PANE,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn is(&self, o: Self) -> bool {
        *self == o
    }
}

#[inline]
pub(crate) fn expand(
    ast: &mut DeriveInput,
    layout_meta: &Meta,
    layout: &str,
    ignore_default: bool,
    is_popup: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    gen_layout_clause(
        ast,
        LayoutType::from(layout_meta, layout)?,
        ignore_default,
        is_popup,
    )
}

/// Get the fileds' Ident which defined the attribute `#[children]` provided by the derive macro [`Childrenable`](crate::Childrenable)
fn get_childrened_fields(ast: &DeriveInput) -> Vec<Ident> {
    let mut children_idents = vec![];
    match &ast.data {
        syn::Data::Struct(ref struct_data) => match &struct_data.fields {
            syn::Fields::Named(syn::FieldsNamed { ref named, .. }) => {
                for field in named.iter() {
                    for attr in field.attrs.iter() {
                        if let Some(attr_ident) = attr.path.get_ident() {
                            if *attr_ident == "children" {
                                children_idents.push(field.ident.clone().unwrap());
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

fn gen_layout_clause(
    ast: &mut DeriveInput,
    layout: LayoutType,
    ignore_default: bool,
    is_popup: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    use LayoutType::*;
    let has_content_alignment = layout == VBox || layout == HBox;
    let has_size_unified_adjust = layout == VBox || layout == HBox || layout == SplitPane;
    let is_hbox = layout == HBox;
    let is_vbox = layout == VBox;
    let is_split_pane = layout == SplitPane;
    let is_stack = layout == Stack;
    let is_scroll_area = layout == ScrollArea;
    let is_pane = layout == Pane;

    let children_fields = get_childrened_fields(ast);

    let mut token = extend_container::expand(
        ast,
        ignore_default,
        false,
        has_content_alignment,
        has_size_unified_adjust,
        layout,
        Some(&children_fields),
        is_popup,
    )?;

    let name = &ast.ident;
    let span = ast.span();
    let layout_ident = Ident::new(layout.as_str(), name.span());

    if is_split_pane && !children_fields.is_empty() {
        return Err(syn::Error::new_spanned(
            &children_fields[0],
            "`SplitPane` can not use `#[children]` attribute, please use `add_child`, `split` functions instead.",
        ));
    }

    let layout = Ident::new(layout.as_str(), span);

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
    } else if is_pane {
        quote!(
            impl SizeUnifiedAdjust for #name {
                #[inline]
                fn size_unified_adjust(&mut self) {
                    Pane::static_size_unified_adjust(self)
                }
            }
        )
    } else {
        proc_macro2::TokenStream::new()
    };

    let impl_spacing_capable = if is_vbox {
        quote!(
            impl SpacingCapable for #name {
                #[inline]
                fn orientation(&self) -> Orientation {
                    Orientation::Vertical
                }
            }
        )
    } else if is_hbox {
        quote!(
            impl SpacingCapable for #name {
                #[inline]
                fn orientation(&self) -> Orientation {
                    Orientation::Horizontal
                }
            }
        )
    } else {
        proc_macro2::TokenStream::new()
    };

    let add_child_clause = match (is_split_pane, is_stack, is_scroll_area, is_pane) {
        (false, false, false, false) => {
            quote! {
                child.set_parent(self);
                ApplicationWindow::initialize_dynamic_component(child.as_mut(), self.is_in_tree());
                self.container.children.push(child);
                self.update();
            }
        }
        (true, _, _, _) => generate_split_pane_add_child()?,
        (_, true, _, _) => generate_stack_add_child()?,
        (_, _, true, _) => generate_scroll_area_add_child(name)?,
        (_, _, _, true) => generate_pane_add_child()?,
    };

    let children_clause = if is_scroll_area {
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

    let impl_split_pane = if is_split_pane {
        generate_split_pane_impl(name)?
    } else {
        proc_macro2::TokenStream::new()
    };

    let impl_stack_trait = if is_stack {
        generate_stack_impl(name)?
    } else {
        proc_macro2::TokenStream::new()
    };

    let impl_scroll_area = if is_scroll_area {
        generate_scroll_area_impl(name)?
    } else {
        proc_macro2::TokenStream::new()
    };

    let impl_pane = if is_pane {
        generate_pane_impl(name)?
    } else {
        proc_macro2::TokenStream::new()
    };

    token.extend(quote!(
        impl ContainerImpl for #name {
            #children_clause

            #[inline]
            fn container_layout(&self) -> ContainerLayoutEnum {
                ContainerLayoutEnum::#layout_ident
            }
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
            #[inline]
            fn composition(&self) -> Composition {
                #layout::static_composition(self)
            }

            #[inline]
            fn position_layout(
                &mut self,
                parent: Option<&dyn WidgetImpl>,
            ) {
                #layout::container_position_layout(self, parent)
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

        #impl_spacing_capable

        #impl_split_pane

        #impl_stack_trait

        #impl_scroll_area

        #impl_pane
    ));

    Ok(token)
}
