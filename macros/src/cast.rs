use quote::{quote, ToTokens};
use syn::{parse::Parse, token, Ident, Token};

pub(crate) struct CastInfo {
    obj: Obj,
    ty: Ident,
}

enum Obj {
    SelfToken(token::SelfValue),
    Ident(Ident),
}

impl ToTokens for Obj {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::SelfToken(t) => t.to_tokens(tokens),
            Self::Ident(i) => i.to_tokens(tokens),
        }
    }
}

impl Parse for CastInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let obj = if lookahead.peek(Token![self]) {
            Obj::SelfToken(input.parse()?)
        } else if lookahead.peek(Ident) {
            Obj::Ident(input.parse()?)
        } else {
            return Err(lookahead.error());
        };

        input.parse::<Token!(as)>()?;

        let ty: Ident = input.parse()?;

        Ok(Self { obj, ty })
    }
}

impl CastInfo {
    pub(crate) fn expand(&self) -> syn::Result<proc_macro2::TokenStream> {
        let obj = &self.obj;
        let mut trait_name = "Reflect".to_string();
        trait_name.push_str(&self.ty.to_string());
        let trait_ident = Ident::new(&trait_name, self.ty.span());
        Ok(quote!(
            if let Some(reflect) = TypeRegistry::get_type_data::<#trait_ident>(#obj.as_reflect()) {
                Some((reflect.get_func)(#obj.as_reflect()))
            } else {
                None
            }
        ))
    }

    pub(crate) fn expand_mut(&self) -> syn::Result<proc_macro2::TokenStream> {
        let obj = &self.obj;
        let mut trait_name = "Reflect".to_string();
        trait_name.push_str(&self.ty.to_string());
        let trait_ident = Ident::new(&trait_name, self.ty.span());
        Ok(quote!(
            if let Some(reflect) = TypeRegistry::get_type_data::<#trait_ident>(#obj.as_reflect()) {
                Some((reflect.get_mut_func)(#obj.as_reflect_mut()))
            } else {
                None
            }
        ))
    }

    pub(crate) fn expand_boxed(&self) -> syn::Result<proc_macro2::TokenStream> {
        let obj = &self.obj;
        let mut trait_name = "Reflect".to_string();
        trait_name.push_str(&self.ty.to_string());
        let trait_ident = Ident::new(&trait_name, self.ty.span());
        Ok(quote!(
            if let Some(reflect) = TypeRegistry::get_type_data::<#trait_ident>(#obj.as_reflect()) {
                Some((reflect.get_boxed_func)(#obj.as_reflect_boxed()))
            } else {
                None
            }
        ))
    }
}
