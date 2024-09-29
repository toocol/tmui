use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, DeriveInput, Error, Token};

pub(crate) struct WinWidget<'a> {
    name: Option<Ident>,
    corr_name: Option<Ident>,
    crs_win_msg: Option<Ident>,
    generics: Option<SplitGenericsRef<'a>>,
}

impl<'a> Parse for WinWidget<'a> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let crs_win_msg: Punctuated<Ident, Token![,]> = input.parse_terminated(Ident::parse)?;
        let mut crs_win_msg: Vec<Ident> = crs_win_msg.into_iter().collect();
        if crs_win_msg.len() > 1 {
            return Err(Error::new(
                input.span(),
                "Cross window message can only set one.",
            ));
        }

        Ok(Self {
            name: None,
            corr_name: None,
            crs_win_msg: crs_win_msg.pop(),
            generics: None,
        })
    }
}

impl<'a> WinWidget<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        let name = &ast.ident;
        Ok(Self {
            name: Some(ast.ident.clone()),
            corr_name: Some(Ident::new(
                &format!("Corr{}", &name.to_string()),
                name.span(),
            )),
            crs_win_msg: None,
            generics: Some(generics),
        })
    }

    pub(crate) fn set_info(&mut self, ast: &DeriveInput, generics: SplitGenericsRef<'a>) {
        let name = &ast.ident;
        self.name = Some(ast.ident.clone());
        self.corr_name = Some(Ident::new(
            &format!("Corr{}", &name.to_string()),
            name.span(),
        ));
        self.generics = Some(generics);
    }

    pub(crate) fn receiver_field_clause(&self) -> Option<TokenStream> {
        if let Some(ref crs_win_msg) = self.crs_win_msg {
            Some(quote!(
                crs_win_receiver: Option<std::sync::mpsc::Receiver<#crs_win_msg>>
            ))
        } else {
            None
        }
    }

    pub(crate) fn cross_win_msg_receiver_impl(&self) -> TokenStream {
        if let Some(_) = self.crs_win_msg {
            let name = self.name.as_ref().unwrap();
            let (impl_generics, ty_generics, where_clause) = self.generics.unwrap();

            quote!(
                impl #impl_generics CrossWinMsgHandlerRequire for #name #ty_generics #where_clause {}

                impl #impl_generics CrossWinMsgHandlerInner for #name #ty_generics #where_clause {
                    fn handle_inner(&mut self) {
                        if self.crs_win_receiver.is_none() {
                            return
                        }
                        while let Ok(msg) = self.crs_win_receiver.as_ref().unwrap().recv() {
                            self.handle(msg)
                        }
                    }
                }
            )
        } else {
            TokenStream::new()
        }
    }

    pub(crate) fn receiver_reflect(&self) -> TokenStream {
        if let Some(_) = self.crs_win_msg.as_ref() {
            let name = self.name.as_ref().unwrap();
            let (_, ty_generics, _) = self.generics.unwrap();
            quote!(
                type_registry.register::<#name #ty_generics, ReflectCrossWinMsgHandlerInner>();
            )
        } else {
            TokenStream::new()
        }
    }

    pub(crate) fn corr_struct_clause(&self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let corr_name = self.corr_name.as_ref().unwrap();

        let channel_field = if let Some(ref crs_win_msg) = self.crs_win_msg {
            quote!(
                sender: Option<std::sync::mpsc::Sender<#crs_win_msg>>,
                receiver: Option<std::sync::mpsc::Receiver<#crs_win_msg>>,
            )
        } else {
            TokenStream::new()
        };

        let sender_impl = if let Some(ref crs_win_msg) = self.crs_win_msg {
            quote!(
                impl CrossWinMsgSender for #corr_name {
                    type T = #crs_win_msg;

                    #[inline]
                    fn send_cross_win_msg(&self, msg: Self::T) {
                        if let Some(ref sender) = self.sender {
                            let _ = sender.send(msg);
                        }
                    }
                }
            )
        } else {
            TokenStream::new()
        };

        let channel_set_clause = if let Some(_) = self.crs_win_msg.as_ref() {
            quote!(
                let (s, r) = std::sync::mpsc::channel();
                w.sender = Some(s);
                w.receiver = Some(r);
            )
        } else {
            TokenStream::new()
        };

        let take_receiver_fn = if let Some(msg) = self.crs_win_msg.as_ref() {
            quote!(
                #[inline]
                pub fn take_receiver(&mut self) -> Option<std::sync::mpsc::Receiver<#msg>> {
                    self.receiver.take()
                }
            )
        } else {
            TokenStream::new()
        };

        let child_proc_fn = if let Some(_) = self.crs_win_msg.as_ref() {
            quote!(
                let receiver = self.take_receiver();
                Box::new(move |win| {
                    let mut w_widget = Object::new::<#name>(&[]);
                    w_widget.crs_win_receiver = receiver;
                    win.child(w_widget)
                })
            )
        } else {
            quote!(
                Box::new(|win| {
                    win.child(Object::new::<#name>(&[]))
                })
            )
        };

        quote!(
            #[extends(Widget)]
            pub struct #corr_name {
                #channel_field
            }

            #sender_impl

            impl ObjectSubclass for #corr_name {
                const NAME: &'static str = stringify!(#corr_name);
            }

            impl ObjectImpl for #corr_name {
                #[inline]
                fn type_register(&self, type_registry: &mut TypeRegistry) {
                    type_registry.register::<#corr_name, ReflectWinWidget>();
                }
            }

            impl WidgetImpl for #corr_name {}

            impl WinWidget for #corr_name {
                #[inline]
                fn child_process_fn(&self) -> Box<dyn Fn(&mut ApplicationWindow) + Send> {
                    #child_proc_fn
                }
            }

            impl #corr_name {
                #[inline]
                pub fn new() -> Box<Self> {
                    let mut w: Box<#corr_name> = Object::new(&[]);
                    #channel_set_clause
                    w
                }

                #take_receiver_fn
            }
        )
    }
}
