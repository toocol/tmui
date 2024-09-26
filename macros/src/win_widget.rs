use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub(crate) struct WinWidget<'a> {
    name: Ident,
    corr_name: Ident,
    generics: SplitGenericsRef<'a>,
}

impl<'a> WinWidget<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        let name = &ast.ident;
        Ok(Self {
            name: ast.ident.clone(),
            corr_name: Ident::new(&format!("Corr{}", &name.to_string()), name.span()),
            generics,
        })
    }

    pub(crate) fn corr_struct_clause(&self) -> TokenStream {
        let name = &self.name;
        let corr_name = &self.corr_name;
        let (_, ty_generics, _) = self.generics;

        quote!(
            #[extends(Widget)]
            pub struct #corr_name {}

            impl ObjectSubclass for #corr_name {
                const NAME: &'static str = stringify!(#corr_name);
            }

            impl ObjectImpl for #corr_name {
                #[inline]
                fn type_register(&self, type_registry: &mut TypeRegistry) {
                    type_registry.register::<#corr_name #ty_generics, ReflectWinWidget>();
                }
            }

            impl WidgetImpl for #corr_name {}

            impl WinWidget for #corr_name {
                #[inline]
                fn child_process_fn(&self) -> Box<dyn Fn(&mut ApplicationWindow) + Send + Sync> {
                    Box::new(|win| {
                        win.child(Object::new::<#name>(&[]))
                    })
                }
            }

            impl #corr_name {
                #[inline]
                pub fn new() -> Box<Self> {
                    Object::new(&[])
                }
            }
        )
    }
}
