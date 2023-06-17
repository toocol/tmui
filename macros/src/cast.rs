use quote::quote;
use syn::{parse::Parse, Ident, Token};

pub(crate) struct CastInfo {
    obj: Ident,
    ty: Ident,
}

impl Parse for CastInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let obj_ident: Ident = input.parse()?;

        input.parse::<Token!(as)>()?;

        let ty_ident: Ident = input.parse()?;

        Ok(Self {
            obj: obj_ident,
            ty: ty_ident,
        })
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
