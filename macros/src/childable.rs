use proc_macro2::Ident;
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Pound, Attribute, FieldsNamed, Path, Token,
};

pub(crate) struct Childable {
    child_field: Option<Ident>,
}

impl Childable {
    #[inline]
    pub(crate) fn new() -> Self {
        Childable { child_field: None }
    }

    pub(crate) fn parse_childable(&mut self, fields: &mut FieldsNamed) -> syn::Result<()> {
        let mut child_field = None;

        // If field with attribute `#[child]`,
        // add attribute `#[derivative(Default(value = "Object::new(&[])"))]` to it,
        for field in fields.named.iter_mut() {
            let mut childable = false;
            for attr in field.attrs.iter() {
                if let Some(attr_ident) = attr.path.get_ident() {
                    if attr_ident.to_string() == "child" {
                        if child_field.is_some() {
                            return Err(syn::Error::new_spanned(
                                field,
                                "Widget can only has one child.",
                            ));
                        }
                        childable = true;
                        break;
                    }
                }
            }

            if childable {
                child_field = Some(field.ident.clone().unwrap());

                let mut segments = Punctuated::<syn::PathSegment, Token![::]>::new();
                segments.push(syn::PathSegment {
                    ident: syn::Ident::new("derivative", field.span()),
                    arguments: syn::PathArguments::None,
                });
                let attr = Attribute {
                    pound_token: Pound {
                        spans: [field.span()],
                    },
                    style: syn::AttrStyle::Outer,
                    bracket_token: syn::token::Bracket { span: field.span() },
                    path: Path {
                        leading_colon: None,
                        segments,
                    },
                    tokens: quote! {(Default(value = "Object::new(&[])"))},
                };
                field.attrs.push(attr);
            }
        }

        self.child_field = child_field;

        Ok(())
    }

    pub(crate) fn get_child_ref(&self) -> proc_macro2::TokenStream {
        match self.child_field.as_ref() {
            Some(field) => {
                quote! {
                    let child = self.#field.as_mut() as *mut dyn WidgetImpl;
                    self._child_ref(child);
                }
            }
            None => proc_macro2::TokenStream::new(),
        }
    }
}
