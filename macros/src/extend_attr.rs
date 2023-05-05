use quote::ToTokens;
use syn::{Ident, Meta, parse::Parse, Token};

/// The attributes in `extends` proc macro, like:
/// `"Widget, Layout(Stack)"` in `#[extends(Widget, Layout(Stack))]`
pub(crate) struct ExtendAttr {
    pub(crate) extend: Ident,
    pub(crate) layout_meta: Option<Meta>,
    pub(crate) layout: Option<String>,
}

impl ExtendAttr {
    fn error<T: ToTokens>(span: T, msg: &'static str) -> syn::Result<Self> {
        Err(syn::Error::new_spanned(span, msg))
    }
}

impl Parse for ExtendAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let extend: Ident = input.parse()?;
        let mut layout = None;
        let layout_meta = if let Some(_) = input.parse::<Option<Token!(,)>>()? {
            let layout_meta = input.parse::<Meta>()?;
            if let Meta::List(syn::MetaList {
                ref path,
                ref nested,
                ..
            }) = layout_meta
            {
                if let Some(ident) = path.get_ident() {
                    if ident.to_string() != "Layout" {
                        return Self::error(
                            layout_meta,
                            "Only support attribute formmat `Layout(xxx)`",
                        );
                    }

                    if nested.len() != 1 {
                        return Self::error(
                            layout_meta,
                            "Only support attribute formmat `Layout(xxx)`",
                        );
                    }

                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = nested.first() {
                        if let Some(layout_ident) = path.get_ident() {
                            layout = Some(layout_ident.to_string())
                        }
                    } else {
                        return Self::error(
                            layout_meta,
                            "Only support attribute formmat `Layout(xxx)`",
                        );
                    }
                } else {
                    return Self::error(
                        layout_meta,
                        "Only support attribute formmat `Layout(xxx)`",
                    );
                }
            } else {
                return Self::error(layout_meta, "Only support attribute formmat `Layout(xxx)`");
            }
            Some(layout_meta)
        } else {
            None
        };
        Ok(Self {
            extend,
            layout_meta,
            layout,
        })
    }
}
