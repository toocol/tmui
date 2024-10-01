use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, DeriveInput, Error, Token};

pub(crate) struct WinWidget<'a> {
    name: Option<Ident>,
    corr_name: Option<Ident>,
    crs_win_msg: Option<Ident>,
    generics: Option<SplitGenericsRef<'a>>,
    is_popup: bool,
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
            crs_win_msg: None,
            generics: Some(generics),
            is_popup: false,
        })
    }

    pub(crate) fn set_info(&mut self, ast: &DeriveInput, generics: SplitGenericsRef<'a>, is_popup: bool) {
        let name = &ast.ident;
        self.name = Some(ast.ident.clone());
        self.corr_name = Some(Ident::new(
            &format!("Corr{}", &name.to_string()),
            name.span(),
        ));
        self.generics = Some(generics);
        self.is_popup = is_popup;
    }

    pub(crate) fn sink_field_clause(&self) -> Option<TokenStream> {
        self.crs_win_msg.as_ref().map(|crs_win_msg| {
            quote!(
                crs_sink_receiver: Option<std::sync::mpsc::Receiver<#crs_win_msg>>
            )
        })
    }

    pub(crate) fn sink_impl(&self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let (impl_generics, ty_generics, where_clause) = self.generics.unwrap();

        if self.crs_win_msg.is_some() {
            quote!(
                impl #impl_generics CrossWinWidget for #name #ty_generics #where_clause {}

                impl #impl_generics CrossWinMsgHandlerRequire for #name #ty_generics #where_clause {}

                impl #impl_generics CrossWinMsgHandlerInner for #name #ty_generics #where_clause {
                    fn handle_inner(&mut self) {
                        if self.crs_sink_receiver.is_none() {
                            return
                        }
                        while let Ok(msg) = self.crs_sink_receiver.as_ref().unwrap().try_recv() {
                            self.handle(msg)
                        }
                    }
                }
            )
        } else {
            quote!(
                impl #impl_generics CrossWinWidget for #name #ty_generics #where_clause {}
            )
        }
    }

    pub(crate) fn sink_reflect(&self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let (_, ty_generics, _) = self.generics.unwrap();

        if self.crs_win_msg.is_some() {
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

        let channel_field = if let Some(ref crs_win_msg) = self.crs_win_msg {
            quote!(
                crs_sink_sender: Option<std::sync::mpsc::Sender<#crs_win_msg>>,
                crs_sink_receiver: Option<std::sync::mpsc::Receiver<#crs_win_msg>>,
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
                        if let Some(ref sender) = self.crs_sink_sender {
                            let _ = sender.send(msg);
                        }
                    }
                }
            )
        } else {
            TokenStream::new()
        };

        let channel_set_clause = if self.crs_win_msg.is_some() {
            quote!(
                let (s, r) = std::sync::mpsc::channel();
                w.crs_sink_sender = Some(s);
                w.crs_sink_receiver = Some(r);
            )
        } else {
            TokenStream::new()
        };

        let take_receiver_fn = if let Some(msg) = self.crs_win_msg.as_ref() {
            quote!(
                #[inline]
                pub fn take_sink_receiver(&mut self) -> Option<std::sync::mpsc::Receiver<#msg>> {
                    self.crs_sink_receiver.take()
                }
            )
        } else {
            TokenStream::new()
        };

        let child_proc_fn = if self.crs_win_msg.is_some() {
            quote!(
                let receiver = self.take_sink_receiver();
                Box::new(move |win| {
                    let mut w_widget = Object::new::<#name>(&[]);
                    w_widget.crs_sink_receiver = receiver;
                    win.child(w_widget);
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
            #extends_clause
            pub struct #corr_name {
                #channel_field
            }

            #sender_impl

            #popup_impl_clause

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
            }
        )
    }
}
