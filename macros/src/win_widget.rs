use crate::SplitGenericsRef;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, DeriveInput, Error, Lit, Meta,
    MetaNameValue, Token, Type,
};

pub(crate) struct WinWidget<'a> {
    name: Option<Ident>,
    corr_name: Option<Ident>,
    /// Origin to Sink message.
    crs_o2s_msg: Option<Ident>,
    /// Sink to Origin message,
    crs_s2o_msg: Option<Ident>,
    generics: Option<SplitGenericsRef<'a>>,
    is_popup: bool,

    // PopupImpl functions define:
    calculate_position: Option<TokenStream>,
    is_modal: Option<TokenStream>,
    hide_on_click: Option<TokenStream>,
    move_capable: Option<TokenStream>,
    handle_global_mouse_pressed: Option<TokenStream>,
    on_mouse_click_hide: Option<TokenStream>,
    on_win_size_change: Option<TokenStream>,

    // Correspondent widget fields:
    fields: TokenStream,
}

impl Parse for WinWidget<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut crs_o2s_msg = None;
        let mut crs_s2o_msg = None;
        let metas: Punctuated<Meta, Token![,]> = input.parse_terminated(Meta::parse)?;
        let metas: Vec<Meta> = metas.into_iter().collect();

        let mut calculate_position: Option<TokenStream> = None;
        let mut is_modal: Option<TokenStream> = None;
        let mut hide_on_click: Option<TokenStream> = None;
        let mut move_capable: Option<TokenStream> = None;
        let mut handle_global_mouse_pressed: Option<TokenStream> = None;
        let mut on_mouse_click_hide: Option<TokenStream> = None;
        let mut on_win_size_change: Option<TokenStream> = None;

        let mut fields = TokenStream::new();

        for meta in metas.into_iter() {
            match meta {
                Meta::Path(path) => {
                    if let Some(ident) = path.get_ident() {
                        if crs_o2s_msg.is_none() {
                            crs_o2s_msg = Some(ident.clone())
                        } else if crs_s2o_msg.is_none() {
                            crs_s2o_msg = Some(ident.clone())
                        } else {
                            return Err(Error::new(
                                input.span(),
                                "Can noly set 2 cross window message.",
                            ));
                        }
                    } else {
                        return Err(Error::new(
                            input.span(),
                            "Unsupported WinWidget configuration.",
                        ));
                    }
                }

                Meta::NameValue(syn::MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    let ident = path.get_ident().unwrap();

                    match ident.to_string().as_str() {
                        "o2s" => match lit {
                            syn::Lit::Str(lit_str) => {
                                if crs_o2s_msg.is_some() {
                                    return Err(Error::new(
                                        input.span(),
                                        "o2s Message have been setted.",
                                    ));
                                }
                                crs_o2s_msg = Some(Ident::new(&lit_str.value(), input.span()))
                            }
                            _ => {
                                return Err(Error::new(
                                    input.span(),
                                    "Value of `o2s` should be string.",
                                ))
                            }
                        },
                        "s2o" => match lit {
                            syn::Lit::Str(lit_str) => {
                                if crs_s2o_msg.is_some() {
                                    return Err(Error::new(
                                        input.span(),
                                        "s2o Message have been setted.",
                                    ));
                                }
                                crs_s2o_msg = Some(Ident::new(&lit_str.value(), input.span()))
                            }
                            _ => {
                                return Err(Error::new(
                                    input.span(),
                                    "Value of `s2o` should be string.",
                                ))
                            }
                        },
                        s => {
                            return Err(Error::new(
                                input.span(),
                                format!("Unsupported WinWidget configuration: {}", s),
                            ))
                        }
                    }
                }

                Meta::List(syn::MetaList {
                    ref path,
                    ref nested,
                    ..
                }) => {
                    if let Some(ident) = path.get_ident() {
                        if *ident != "o2s"
                            && *ident != "s2o"
                            && *ident != "PopupImpl"
                            && *ident != "fields"
                        {
                            return Err(Error::new(
                                input.span(),
                                "Unsupported WinWidget configuration, Unkonwn config, only o2s(xx), s2o(xx) is supported.",
                            ));
                        }

                        if *ident != "PopupImpl" && *ident != "fields" && nested.len() != 1 {
                            return Err(Error::new(
                                input.span(),
                                "Unsupported WinWidget configuration, two many args, only o2s(xx), s2o(xx) is supported.",
                            ));
                        }

                        if *ident == "fields" {
                            for meta in nested.iter() {
                                if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                    path,
                                    lit,
                                    ..
                                })) = meta
                                {
                                    let field_ident = path;
                                    let field_ty = if let Lit::Str(lit_str) = lit {
                                        syn::parse_str::<Type>(&lit_str.value())?
                                    } else {
                                        return Err(Error::new(
                                            input.span(),
                                            "Unsupported WinWidget configuration, field type can only be LitStr.",
                                        ));
                                    };

                                    fields.extend(quote!(
                                        #field_ident: #field_ty,
                                    ));
                                } else {
                                    return Err(Error::new(
                                        meta.span(),
                                        "Unsupported WinWidget configuration, field config format is `some_field = \"SomeType\"`.",
                                    ));
                                }
                            }
                        }

                        if *ident == "PopupImpl" {
                            for meta in nested.iter() {
                                if let syn::NestedMeta::Meta(Meta::List(meta_list)) = meta {
                                    let fn_ident = meta_list.path.get_ident().unwrap();
                                    let fn_call =
                                        if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) =
                                            meta_list.nested.first()
                                        {
                                            path.get_ident().unwrap().clone()
                                        } else {
                                            return Err(Error::new(
                                            input.span(),
                                            "PopupImpl function call only support literal string.",
                                        ));
                                        };

                                    match fn_ident.to_string().as_str() {
                                        "calculate_position" => {
                                            calculate_position = Some(quote!(
                                                #[inline]
                                                fn calculate_position(&self, base_rect: Rect, mut point: Point) -> Point {
                                                    #fn_call(self, base_rect, point)
                                                }
                                            ))
                                        }
                                        "is_modal" => {
                                            is_modal = Some(quote!(
                                                #[inline]
                                                fn is_modal(&self) -> bool {
                                                    #fn_call(self)
                                                }
                                            ))
                                        }
                                        "hide_on_click" => {
                                            hide_on_click = Some(quote!(
                                                #[inline]
                                                fn hide_on_click(&self) -> bool {
                                                    #fn_call(self)
                                                }
                                            ))
                                        }
                                        "move_capable" => {
                                            move_capable = Some(quote!(
                                                #[inline]
                                                fn move_capable(&self) -> bool {
                                                    #fn_call(self)
                                                }
                                            ))
                                        }
                                        "handle_global_mouse_pressed" => {
                                            handle_global_mouse_pressed = Some(quote!(
                                                #[inline]
                                                fn handle_global_mouse_pressed(&mut self, evt: &MouseEvent) -> bool {
                                                    #fn_call(self, evt)
                                                }
                                            ))
                                        }
                                        "on_mouse_click_hide" => {
                                            on_mouse_click_hide = Some(quote!(
                                                #[inline]
                                                fn on_mouse_click_hide(&mut self) -> bool {
                                                    #fn_call(self)
                                                }
                                            ))
                                        }
                                        "on_win_size_change" => {
                                            on_win_size_change = Some(quote!(
                                                #[inline]
                                                fn on_win_size_change(&mut self, size: Size) -> bool {
                                                    #fn_call(self, size)
                                                }
                                            ))
                                        }
                                        _ => {
                                            return Err(Error::new(
                                                input.span(),
                                                "Unkown function name in PopupImpl",
                                            ))
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = nested.first() {
                            if let Some(attr_ident) = path.get_ident() {
                                if *ident == "o2s" {
                                    if crs_o2s_msg.is_some() {
                                        return Err(Error::new(
                                            input.span(),
                                            "o2s Message have been setted.",
                                        ));
                                    }

                                    crs_o2s_msg = Some(attr_ident.clone())
                                }

                                if *ident == "s2o" {
                                    if crs_s2o_msg.is_some() {
                                        return Err(Error::new(
                                            input.span(),
                                            "s2o Message have been setted.",
                                        ));
                                    }
                                    crs_s2o_msg = Some(attr_ident.clone())
                                }
                            }
                        } else if *ident != "PopupImpl" && *ident != "fields" {
                            return Err(Error::new(
                                input.span(),
                            "Unsupported WinWidget configuration, nested meta is None, only o2s(xx), s2o(xx) is supported.",
                            ));
                        }
                    } else {
                        return Err(Error::new(
                            input.span(),
                            "Unsupported WinWidget configuration.",
                        ));
                    }
                }
            }
        }

        Ok(Self {
            name: None,
            corr_name: None,
            crs_o2s_msg,
            crs_s2o_msg,
            generics: None,
            is_popup: false,
            calculate_position,
            is_modal,
            hide_on_click,
            move_capable,
            handle_global_mouse_pressed,
            on_mouse_click_hide,
            on_win_size_change,
            fields,
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
            calculate_position: None,
            is_modal: None,
            hide_on_click: None,
            move_capable: None,
            handle_global_mouse_pressed: None,
            on_mouse_click_hide: None,
            on_win_size_change: None,
            fields: TokenStream::new(),
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

    pub(crate) fn corr_struct_clause(&mut self) -> TokenStream {
        let name = self.name.as_ref().unwrap();
        let corr_name = self.corr_name.as_ref().unwrap();

        let extends_clause = if self.is_popup {
            quote!(#[extends(Popup)])
        } else {
            quote!(#[extends(Widget)])
        };

        // PopupImpl function override:
        let popup_impl_clause = if self.is_popup {
            let calculate_position = self.calculate_position.take().unwrap_or_default();
            let is_modal = self.is_modal.take().unwrap_or_default();
            let hide_on_click = self.hide_on_click.take().unwrap_or_default();
            let move_capable = self.move_capable.take().unwrap_or_default();
            let handle_global_mouse_pressed =
                self.handle_global_mouse_pressed.take().unwrap_or_default();
            let on_mouse_click_hide = self.on_mouse_click_hide.take().unwrap_or_default();
            let on_win_size_change = self.on_win_size_change.take().unwrap_or_default();
            quote!(
                impl PopupImpl for #corr_name {
                    #calculate_position

                    #is_modal

                    #hide_on_click

                    #move_capable

                    #handle_global_mouse_pressed

                    #on_mouse_click_hide

                    #on_win_size_change
                }
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
                        } else {
                            log::warn!("The cross window message sender is None. {}", self.name());
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
                    let mut w_widget = #name::new_alloc();
                    w_widget.set_background(Color::WHITE);
                    #set_receiver_clause
                    #set_sender_clause
                    win.child(w_widget);
                })
            ));
            token
        } else {
            quote!(
                Box::new(|win| {
                    win.child(#name::new_alloc())
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

        let corr_fields = &self.fields;

        quote!(
            #extends_clause
            pub struct #corr_name {
                #channel_field

                #corr_fields
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
                pub fn new() -> Tr<Self> {
                    let mut w: Tr<#corr_name> = Self::new_alloc();
                    #channel_set_clause
                    w
                }

                #take_receiver_fn
                #take_sender_fn
            }
        )
    }
}
