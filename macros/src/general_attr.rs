use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{animation::Animation, async_task::AsyncTask};

pub(crate) struct GeneralAttr {
    // fields about `run_after`
    pub(crate) run_after_clause: TokenStream,

    // fields about `animation`
    pub(crate) is_animation: bool,
    pub(crate) animation: Option<Animation>,
    pub(crate) animation_clause: TokenStream,
    pub(crate) animation_field: TokenStream,

    // field about `async_task`
    pub(crate) is_async_task: bool,
    pub(crate) async_task_fields: Vec<TokenStream>,
    pub(crate) async_task_impl_clause: TokenStream,
    pub(crate) async_task_method_clause: TokenStream,
}

impl GeneralAttr {
    pub(crate) fn parse(ast: &DeriveInput) -> syn::Result<Self> {
        let name = &ast.ident;

        let mut is_run_after = false;
        let mut is_animation = false;
        let mut animation = None;
        let mut is_async_task = false;
        let mut async_tasks = vec![];

        for attr in ast.attrs.iter() {
            if let Some(attr_ident) = attr.path.get_ident() {
                let attr_str = attr_ident.to_string();

                match attr_str.as_str() {
                    "run_after" => is_run_after = true,
                    "animatable" => animation = {
                        is_animation = true;
                        Some(Animation::parse(attr)?)
                    },
                    "async_task" => {
                        is_async_task = true;
                        async_tasks.push(AsyncTask::parse_attr(attr));
                    }
                    _ => {}
                }
            }
        }

        // Run after:
        let run_after_clause = if is_run_after {
            quote!(
                ApplicationWindow::run_afters_of(self.window_id()).push(
                    std::ptr::NonNull::new(self)
                );
            )
        } else {
            proc_macro2::TokenStream::new()
        };

        // Animation:
        let animation_clause = if let Some(animation) = animation.as_ref() {
            animation.generate_animation(name)?
        } else {
            proc_macro2::TokenStream::new()
        };
        let animation_field = if animation.is_some() {
            quote!(
                pub animation: AnimationModel
            )
        } else {
            proc_macro2::TokenStream::new()
        };

        // Async task:
        let mut async_task_fields = vec![];
        if is_async_task {
            for async_task in async_tasks.iter() {
                if async_task.is_none() {
                    return Err(syn::Error::new_spanned(
                        ast,
                        "proc_macro `async_task` format error.",
                    ));
                }
                let task = async_task.as_ref().unwrap();
                let task_name = task.name.as_ref().unwrap();
                let field = task.field.as_ref().unwrap();

                async_task_fields.push(quote! {
                    #field: Option<Box<#task_name>>
                });
            }
        }

        let async_task_impl_clause = if is_async_task {
            let mut clause = proc_macro2::TokenStream::new();
            for async_task in async_tasks.iter() {
                clause.extend(async_task.as_ref().unwrap().expand(ast)?)
            }
            clause
        } else {
            proc_macro2::TokenStream::new()
        };

        let async_task_method_clause = if is_async_task {
            let mut clause = proc_macro2::TokenStream::new();
            for async_task in async_tasks.iter() {
                clause.extend(async_task.as_ref().unwrap().expand_method(ast)?)
            }
            clause
        } else {
            proc_macro2::TokenStream::new()
        };

        Ok(Self {
            run_after_clause,
            is_animation,
            animation,
            animation_clause,
            animation_field,
            is_async_task,
            async_task_fields,
            async_task_impl_clause,
            async_task_method_clause,
        })
    }
}
