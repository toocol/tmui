use proc_macro2::Ident;
use quote::quote;
use syn::{
    Attribute, DeriveInput, Meta, MetaList, MetaNameValue, NestedMeta,
};

pub(crate) struct AsyncTask {
    pub(crate) name: Option<Ident>,
    pub(crate) name_snake: Option<Ident>,

    pub(crate) value: Option<Ident>,

    pub(crate) field: Option<Ident>,
}

impl AsyncTask {
    pub(crate) fn parse_attr(attr: &Attribute) -> Option<Self> {
        let mut res = Self {
            name: None,
            name_snake: None,
            value: None,
            field: None,
        };

        if let Ok(meta) = attr.parse_meta() {
            match meta {
                Meta::List(MetaList { nested, .. }) => {
                    for meta in nested {
                        match meta {
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                ref path,
                                ref lit,
                                ..
                            })) => {
                                let ident = path.get_ident().unwrap();

                                match ident.to_string().as_str() {
                                    "name" => match lit {
                                        syn::Lit::Str(lit) => {
                                            let name = lit.value();
                                            let ident = Ident::new(&name, lit.span());
                                            res.name = Some(ident);

                                            let snake_name = to_snake_case(&name);

                                            res.name_snake = Some(Ident::new(&snake_name, lit.span()));
                                            res.field = Some(Ident::new(
                                                &format!("task_{}", snake_name),
                                                lit.span(),
                                            ));
                                        }
                                        _ => return None,
                                    },
                                    "value" => match lit {
                                        syn::Lit::Str(lit) => {
                                            let ident = Ident::new(&lit.value(), lit.span());
                                            res.value = Some(ident)
                                        }
                                        _ => return None,
                                    },
                                    _ => return None,
                                }
                            }
                            _ => return None,
                        }
                    }
                }
                _ => {}
            }
        }

        Some(res)
    }

    pub(crate) fn expand(&self, ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
        if self.name.is_none() || self.value.is_none() {
            return Err(syn::Error::new_spanned(
                ast,
                "`async_task` should have both attribute `name` and `value` defined.",
            ));
        }

        let widget = &ast.ident;

        match &ast.data {
            syn::Data::Struct(..) => {
                let task = self.name.as_ref().unwrap();
                let value = self.value.as_ref().unwrap();

                Ok(quote!(
                    #[extends(Object)]
                    struct #task {
                        join_handler: Option<tlib::tokio::task::JoinHandle<#value>>,
                        timer: Box<tlib::timer::Timer>,
                        then: Option<Box<dyn FnOnce(&'static mut #widget, #value)>>,
                        widget: Option<std::ptr::NonNull<#widget>>,
                    }

                    impl ObjectSubclass for #task {
                        const NAME: &'static str = "AsyncTask";
                    }
                    impl ObjectImpl for #task {}

                    impl #task {
                        #[inline]
                        pub fn new(widget: &mut #widget, join: tlib::tokio::task::JoinHandle<#value>) -> Box<Self> {
                            let mut task = Box::new(Self {
                                object: Default::default(),
                                join_handler: Some(join),
                                timer: tlib::timer::Timer::new(),
                                then: None,
                                widget: std::ptr::NonNull::new(widget),
                            });

                            tlib::connect!(task.timer, timeout(), task, check());
                            task.timer.start(std::time::Duration::from_millis(1));

                            task
                        }

                        #[inline]
                        pub fn then<F: FnOnce(&'static mut #widget, #value) + 'static>(mut self: Box<Self>, then: F) -> Box<Self> {
                            self.then = Some(Box::new(then));
                            self
                        }

                        pub fn check(&mut self) {
                            let join_handler = self.join_handler.as_mut().unwrap();
                            if join_handler.is_finished() {
                                self.timer.disconnect_all();
                                self.timer.stop();

                                let result = tlib::r#async::tokio_runtime().block_on(join_handler).unwrap();
                                if let Some(then) = self.then.take() {
                                    then(tlib::nonnull_mut!(self.widget), result);
                                }
                            }
                        }
                    }
                ))
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    ast,
                    "`async_task` should defined on struct.",
                ))
            }
        }
    }

    pub(crate) fn expand_method(&self, ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
        if self.name.is_none() || self.value.is_none() {
            return Err(syn::Error::new_spanned(
                ast,
                "`async_task` should have both attribute `name` and `value` defined.",
            ));
        }
        let widget = &ast.ident;
        let name = self.name.as_ref().unwrap();
        let name_snake = self.name_snake.as_ref().unwrap();
        let value = self.value.as_ref().unwrap();
        let field = self.field.as_ref().unwrap();

        Ok(quote!(
            fn #name_snake<F, T>(&mut self, future: F, then: Option<T>)
            where
                F: std::future::Future<Output = #value> + Send + 'static,
                T: FnOnce(&'static mut #widget, #value) + 'static,
            {
                let join = tlib::tokio::spawn(future);
                let mut task = #name::new(self, join);
                if let Some(then) = then {
                    task = task.then(then);
                }
                self.#field = Some(task);
            }
        ))
    }
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let mut chars = s.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_uppercase() {
            if !snake_case.is_empty() {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        } else {
            snake_case.push(c);
        }
        chars.next();
    }

    snake_case
}
