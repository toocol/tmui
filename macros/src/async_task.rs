use proc_macro2::Ident;
use quote::quote;
use syn::{
    Attribute, DeriveInput, Meta, MetaList, MetaNameValue, NestedMeta,
};

pub(crate) struct AsyncTask {
    pub(crate) name: Option<Ident>,

    pub(crate) value: Option<Ident>,

    pub(crate) field: Option<Ident>,
}

impl AsyncTask {
    pub(crate) fn parse_attr(attr: &Attribute) -> Option<Self> {
        let mut res = Self {
            name: None,
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
                                            res.field = Some(Ident::new(
                                                &format!("task_{}", to_snake_case(&name),),
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
        match &ast.data {
            syn::Data::Struct(..) => {
                let task = self.name.as_ref().unwrap();
                let value = self.value.as_ref().unwrap();

                Ok(quote!(
                    #[extends(Object)]
                    struct #task {
                        join_handler: Option<tlib::tokio::task::JoinHandle<#value>>,
                        timer: Box<tlib::timer::Timer>,
                        then: Option<Box<dyn FnOnce(#value)>>,
                    }

                    impl ObjectSubclass for #task {
                        const NAME: &'static str = "AsyncTask";
                    }
                    impl ObjectImpl for #task {}

                    impl #task {
                        #[inline]
                        pub fn new(join: tlib::tokio::task::JoinHandle<#value>) -> Box<Self> {
                            let mut task = Box::new(Self {
                                object: Default::default(),
                                join_handler: Some(join),
                                timer: tlib::timer::Timer::new(),
                                then: None,
                            });

                            tlib::connect!(task.timer, timeout(), task, check());
                            task.timer.start(std::time::Duration::from_millis(1));

                            task
                        }

                        #[inline]
                        pub fn then<F: FnOnce(#value) + 'static>(mut self: Box<Self>, then: F) -> Box<Self> {
                            self.then = Some(Box::new(then));
                            self
                        }

                        pub fn check(&mut self) {
                            let join_handler = self.join_handler.as_mut().unwrap();
                            if join_handler.is_finished() {
                                self.timer.disconnect_all();
                                self.timer.stop();

                                let result = tokio_runtime().block_on(join_handler).unwrap();
                                if let Some(then) = self.then.take() {
                                    then(result);
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
