use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, DeriveInput, Error, Token};

pub(crate) struct WinWidget<'a> {
    name: Option<Ident>,
    corr_name: Option<Ident>,
    /// Origin to Sink message.
    crs_o2s_msg: Option<Ident>,
    /// Sink to Origin message,
    crs_s2o_msg: Option<Ident>,
    generics: Option<SplitGenericsRef<'a>>,
    is_popup: bool,
}

impl<'a> Parse for WinWidget<'a> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let crs_win_msg: Punctuated<Ident, Token![,]> = input.parse_terminated(Ident::parse)?;
        let mut crs_win_msg: Vec<Ident> = crs_win_msg.into_iter().collect();
        if crs_win_msg.len() > 2 {
            return Err(Error::new(
                input.span(),
                "The maximum count of cross window message is 2.",
            ));
        }

        Ok(Self {
            name: None,
            corr_name: None,
            crs_o2s_msg: crs_win_msg.pop(),
            crs_s2o_msg: crs_win_msg.pop(),
            generics: None,
            is_popup: false,
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
            crs_o2s_msg: None,
            crs_s2o_msg: None,
            generics: Some(generics),
            is_popup: false,
        })
    }

    pub(crate) fn set_info(
        &mut self,
        ast: &DeriveInput,
        generics: SplitGenericsRef<'a>,
        is_popup: bool,
    ) {
        let name = &ast.ident;
        self.name = Some(ast.ident.clone());
        self.corr_name = Some(Ident::new(
            &format!("Corr{}", &name.to_string()),
            name.span(),
        ));
        self.generics = Some(generics);
        self.is_popup = is_popup;
    }

    pub(crate) fn sink_field_clause(&self) -> Vec<TokenStream> {
        let mut fields = vec![];

        if let Some(msg) = self.crs_o2s_msg.as_ref() {
            fields.push(quote!(
                crs_sink_receiver: Option<std::sync::mpsc::Receiver<#msg>>
            ));
        }

        if let Some(msg) = self.crs_s2o_msg.as_ref() {
            fields.push(quote!(
                crs_origin_sender: Option<std::sync::mpsc::Sender<#msg>>
            ));
        }

        fields
    }

    pub(crate) fn sink_impl(&self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let (impl_generics, ty_generics, where_clause) = self.generics.unwrap();
        let mut token = quote!(
            impl #impl_generics CrossWinWidget for #name #ty_generics #where_clause {}
        );

        if self.crs_o2s_msg.is_some() {
            token.extend(quote!(
                impl #impl_generics CrossWinMsgHandlerRequire for #name #ty_generics #where_clause {}

                impl #impl_generics CrossWinMsgHandlerInner for #name #ty_generics #where_clause {
                    #[inline]
                    fn handle_inner(&mut self) {
                        if self.crs_sink_receiver.is_none() {
                            return
                        }
                        while let Ok(msg) = self.crs_sink_receiver.as_ref().unwrap().try_recv() {
                            self.handle(msg)
                        }
                    }
                }
            ));
        }

        if let Some(msg) = self.crs_s2o_msg.as_ref() {
            token.extend(quote!(
                impl #impl_generics CrossWinMsgSender for #name #ty_generics #where_clause {
                    type T = #msg;

                    #[inline]
                    fn send_cross_win_msg(&self, msg: Self::T) {
                        if let Some(ref sender) = self.crs_origin_sender {
                            let _ = sender.send(msg);
                        }
                    }
                }
            ));
        }

        token
    }

    pub(crate) fn sink_reflect(&self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let (_, ty_generics, _) = self.generics.unwrap();

        if self.crs_o2s_msg.is_some() {
            quote!(
                type_registry.register::<#name #ty_generics, ReflectCrossWinWidget>();
                type_registry.register::<#name #ty_generics, ReflectCrossWinMsgHandlerInner>();
            )
        } else {
            quote!(
                type_registry.register::<#name #ty_generics, ReflectCrossWinWidget>();
            )
        }
    }

    pub(crate) fn corr_struct_clause(&self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let corr_name = self.corr_name.as_ref().unwrap();

        let extends_clause = if self.is_popup {
            quote!(#[extends(Popup)])
        } else {
            quote!(#[extends(Widget)])
        };

        let popup_impl_clause = if self.is_popup {
            quote!(
                impl PopupImpl for #corr_name {}
            )
        } else {
            TokenStream::new()
        };

        let mut channel_field = if let Some(ref crs_win_msg) = self.crs_o2s_msg {
            quote!(
                crs_sink_sender: Option<std::sync::mpsc::Sender<#crs_win_msg>>,
                crs_sink_receiver: Option<std::sync::mpsc::Receiver<#crs_win_msg>>,
            )
        } else {
            TokenStream::new()
        };

        if let Some(ref msg) = self.crs_s2o_msg {
            channel_field.extend(quote!(
                crs_origin_sender: Option<std::sync::mpsc::Sender<#msg>>,
                crs_origin_receiver: Option<std::sync::mpsc::Receiver<#msg>>,
            ));
        }

        let sender_impl = if let Some(ref crs_win_msg) = self.crs_o2s_msg {
            quote!(
                impl CrossWinMsgSender for #corr_name {
                    type T = #crs_win_msg;

                    #[inline]
                    fn send_cross_win_msg(&self, msg: Self::T) {
                        if let Some(ref sender) = self.crs_sink_sender {
                            let _ = sender.send(msg);
                        }
                    }
                }
            )
        } else {
            TokenStream::new()
        };

        let receiver_impl = if self.crs_s2o_msg.is_some() {
            quote!(
                impl CrossWinMsgHandlerRequire for #corr_name {}

                impl CrossWinMsgHandlerInner for #corr_name {
                    #[inline]
                    fn handle_inner(&mut self) {
                        if self.crs_origin_receiver.is_none() {
                            return
                        }
                        while let Ok(msg) = self.crs_origin_receiver.as_ref().unwrap().try_recv() {
                            self.handle(msg)
                        }
                    }
                }
            )
        } else {
            TokenStream::new()
        };

        let mut channel_set_clause = if self.crs_o2s_msg.is_some() {
            quote!(
                let (s, r) = std::sync::mpsc::channel();
                w.crs_sink_sender = Some(s);
                w.crs_sink_receiver = Some(r);
            )
        } else {
            TokenStream::new()
        };

        if self.crs_s2o_msg.is_some() {
            channel_set_clause.extend(quote!(
                let (s, r) = std::sync::mpsc::channel();
                w.crs_origin_sender = Some(s);
                w.crs_origin_receiver = Some(r);
            ));
        }

        let take_receiver_fn = if let Some(msg) = self.crs_o2s_msg.as_ref() {
            quote!(
                #[inline]
                pub fn take_sink_receiver(&mut self) -> Option<std::sync::mpsc::Receiver<#msg>> {
                    self.crs_sink_receiver.take()
                }
            )
        } else {
            TokenStream::new()
        };

        let take_sender_fn = if let Some(msg) = self.crs_s2o_msg.as_ref() {
            quote!(
                #[inline]
                pub fn take_origin_sender(&mut self) -> Option<std::sync::mpsc::Sender<#msg>> {
                    self.crs_origin_sender.take()
                }
            )
        } else {
            TokenStream::new()
        };

        let child_proc_fn = if self.crs_o2s_msg.is_some() || self.crs_s2o_msg.is_some() {
            let mut token = TokenStream::new();

            let set_receiver_clause = if self.crs_o2s_msg.is_some() {
                token.extend(quote!(
                    let receiver = self.take_sink_receiver();
                ));
                quote!(
                    w_widget.crs_sink_receiver = receiver;
                )
            } else {
                TokenStream::new()
            };

            let set_sender_clause = if self.crs_s2o_msg.is_some() {
                token.extend(quote!(
                    let sender = self.take_origin_sender();
                ));
                quote!(
                    w_widget.crs_origin_sender = sender;
                )
            } else {
                TokenStream::new()
            };

            token.extend(quote!(
                Box::new(move |win| {
                    let mut w_widget = Object::new::<#name>(&[]);
                    #set_receiver_clause
                    #set_sender_clause
                    win.child(w_widget);
                })
            ));
            token
        } else {
            quote!(
                Box::new(|win| {
                    win.child(Object::new::<#name>(&[]))
                })
            )
        };

        let receiver_reflect = if self.crs_s2o_msg.is_some() {
            quote!(
                type_registry.register::<#corr_name, ReflectCrossWinMsgHandlerInner>();
            )
        } else {
            TokenStream::new()
        };

        quote!(
            #extends_clause
            pub struct #corr_name {
                #channel_field
            }

            #sender_impl
            #receiver_impl

            #popup_impl_clause

            impl ObjectSubclass for #corr_name {
                const NAME: &'static str = stringify!(#corr_name);
            }

            impl ObjectImpl for #corr_name {
                #[inline]
                fn type_register(&self, type_registry: &mut TypeRegistry) {
                    type_registry.register::<#corr_name, ReflectWinWidget>();
                    #receiver_reflect
                }
            }

            impl WidgetImpl for #corr_name {}

            impl WinWidget for #corr_name {
                #[inline]
                fn child_process_fn(&mut self) -> Box<dyn FnOnce(&mut ApplicationWindow) + Send> {
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
                #take_sender_fn
            }
        )
    }
}
