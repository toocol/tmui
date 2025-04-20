use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Pound, Attribute, FieldsNamed,
    GenericArgument, Path, PathArguments, Token, Type, TypePath,
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
        // add attribute `#[derivative(Default(value = "Self::new_alloc()"))]` to it,
        for field in fields.named.iter_mut() {
            let mut ty = None;
            for attr in field.attrs.iter() {
                if let Some(attr_ident) = attr.path.get_ident() {
                    if *attr_ident == "child" {
                        if child_field.is_some() {
                            return Err(syn::Error::new_spanned(
                                field,
                                "Widget can only has one child.",
                            ));
                        }
                        ty = Some(field.ty.clone());
                        break;
                    }
                }
            }

            if let Some(ty) = ty {
                if &get_outer_type_ident_string(&ty) != "Tr" {
                    return Err(syn::Error::new_spanned(
                        field,
                        "Child widget must wrapped with `Tr`.",
                    ));
                }
                if let Some(ty) = extract_inner_type(&ty) {
                    if let Some(ty) = type_to_string_with_turbofish(ty) {
                        child_field = Some(field.ident.clone().unwrap());
                        let value_str = format!("{}::new_alloc()", ty);
                        let value_lit = syn::LitStr::new(&value_str, field.span());

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
                            tokens: quote! {(Default(value = #value_lit))},
                        };
                        field.attrs.push(attr);
                    }
                }
            }
        }

        self.child_field = child_field;

        Ok(())
    }

    pub(crate) fn get_child_ref(&self) -> proc_macro2::TokenStream {
        match self.child_field.as_ref() {
            Some(field) => {
                quote! {
                    let child = self.#field.clone();
                    unsafe { self.child(child) };
                }
            }
            None => proc_macro2::TokenStream::new(),
        }
    }
}

pub fn get_outer_type_ident_string(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty {
        if let Some(last) = type_path.path.segments.last() {
            last.ident.to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

pub fn extract_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        let segment = path.segments.last()?; // e.g., Tr
        if let PathArguments::AngleBracketed(ref args) = segment.arguments {
            for arg in &args.args {
                if let GenericArgument::Type(inner_ty) = arg {
                    return Some(inner_ty);
                }
            }
        }
    }
    None
}
pub fn type_to_string_with_turbofish(ty: &Type) -> Option<String> {
    if let Type::Path(TypePath { path, .. }) = ty {
        let segment = path.segments.last()?;
        if let PathArguments::AngleBracketed(ref args) = segment.arguments {
            let generic_strs: Vec<String> = args
                .args
                .iter()
                .filter_map(|arg| {
                    if let GenericArgument::Type(ty) = arg {
                        Some(ty.to_token_stream().to_string().replace(" ", ""))
                    } else {
                        None
                    }
                })
                .collect();

            let base_type = segment.ident.to_string();
            let generics = generic_strs.join(",");
            return Some(format!("{}::<{}>", base_type, generics));
        }

        Some(segment.ident.to_string())
    } else {
        None
    }
}
