use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, Ident, LitInt, Token};

pub(crate) struct Shortcut {
    keys: Vec<Key>,
}

enum Key {
    Ident(Ident),
    Int(LitInt),
}

impl Parse for Shortcut {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys = Vec::new();

        while !input.is_empty() {
            if let Ok(ident) = input.parse::<Ident>() {
                keys.push(Key::Ident(ident));
            }

            else if let Ok(lit_int) = input.parse::<LitInt>() {
                keys.push(Key::Int(lit_int));
            }

            let _ = input.parse::<Token![+]>();
        }

        Ok(Shortcut { keys })
    }
}

impl Shortcut {
    pub(crate) fn expand(&self) -> syn::Result<TokenStream> {
        let keys = &self.keys;
        let mut clause = TokenStream::new();

        for key in keys {
            match key {
                Key::Ident(key) => {
                    clause.extend(quote!(
                        shortcut.insert(Shortcut::#key);
                    ));
                }
                Key::Int(key) => {
                    let mut name = "Key".to_string();
                    name.push_str(&key.to_string());
                    let key = Ident::new(&name, key.span());
                    clause.extend(quote!(
                        shortcut.insert(Shortcut::#key);
                    ));
                }
            }
        }

        Ok(quote!(
            {
                let mut shortcut = Shortcut::empty();
                #clause
                shortcut
            }
        ))
    }
}
