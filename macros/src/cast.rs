use quote::quote;
use syn::{parse::Parse, Expr, Ident, Token};

pub struct CastInfo {
    obj: Expr,
    ty: Ident,
}

impl Parse for CastInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let obj_expr: Expr = input.parse()?;

        input.parse::<Token![as]>()?;

        let ty_ident: Ident = input.parse()?;

        Ok(Self {
            obj: obj_expr,
            ty: ty_ident,
        })
    }
}

impl CastInfo {
    pub fn expand(&self) -> syn::Result<proc_macro2::TokenStream> {
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
}
