use quote::ToTokens;
use syn::{parse::Parse, Ident, Meta, Token};

/// The attributes in `extends` proc macro, like:
/// `"Widget, Layout(Stack)"` in `#[extends(Widget, Layout(Stack))]`
pub(crate) struct ExtendAttr {
    pub(crate) extend: Ident,

    pub(crate) layout_meta: Option<Meta>,
    pub(crate) layout: Option<String>,

    pub(crate) id_meta: Option<Meta>,
    pub(crate) id: Option<String>,
}

impl ExtendAttr {
    fn error<T: ToTokens>(span: T, msg: &'static str) -> syn::Result<Self> {
        Err(syn::Error::new_spanned(span, msg))
    }

    pub fn check(&self) -> syn::Result<()> {
        let extend_str = self.extend.to_string();
        if extend_str != "Widget" && self.layout_meta.is_some() {
            return Err(syn::Error::new_spanned(
                self.layout_meta.as_ref().unwrap(),
                format!(
                    "`{}` was not supported layout, only `Widget` has layout.",
                    extend_str
                ),
            ));
        }
        if extend_str != "SharedWidget" && self.id_meta.is_some() {
            return Err(syn::Error::new_spanned(
                self.id_meta.as_ref().unwrap(),
                format!("`Id` attribute was supported for `SharedWidget` only."),
            ));
        }
        Ok(())
    }
}

impl Parse for ExtendAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let extend: Ident = input.parse()?;

        let mut extend_attr = Self {
            extend,
            layout_meta: None,
            layout: None,
            id_meta: None,
            id: None,
        };

        while let Some(_) = input.parse::<Option<Token!(,)>>()? {
            let meta = input.parse::<Meta>()?;
            match meta {
                Meta::List(syn::MetaList {
                    ref path,
                    ref nested,
                    ..
                }) => {
                    if let Some(ident) = path.get_ident() {
                        if ident.to_string() != "Layout" {
                            return Self::error(
                                meta,
                                "Only support attribute formmat `Layout(xxx)` for `Widget`.",
                            );
                        }

                        if nested.len() != 1 {
                            return Self::error(
                                meta,
                                "Only support attribute formmat `Layout(xxx)` for `Widget`.",
                            );
                        }

                        if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = nested.first() {
                            if let Some(attr_ident) = path.get_ident() {
                                extend_attr.layout = Some(attr_ident.to_string());
                                extend_attr.layout_meta = Some(meta);
                            }
                        } else {
                            return Self::error(
                                meta,
                                "Only support attribute formmat `Layout(xxx)` for `Widget`.",
                            );
                        }
                    } else {
                        return Self::error(
                            meta,
                            "Only support attribute formmat `Layout(xxx)` for `Widget`.",
                        );
                    }
                }
                Meta::NameValue(syn::MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    let ident = path.get_ident().unwrap();

                    match ident.to_string().as_str() {
                        "id" => {
                            match lit {
                                syn::Lit::Str(lit_str) => {
                                    extend_attr.id = Some(lit_str.value());
                                    extend_attr.id_meta = Some(meta)
                                }
                                _ => return Self::error(meta, "Value of `id` should be string.")
                            }
                        },
                        _ => {
                            return Self::error(
                                meta,
                                "Only support attribute config `id = \"xxx\"` for `SharedWidget`.",
                           )
                        }
                    }
                }
                _ => {
                    return Self::error(
                        meta,
                        "Only support attribute formmat `Layout(xxx)` for `Widget`.",
                    );
                }
            }
        }

        Ok(extend_attr)
    }
}
