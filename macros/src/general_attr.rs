use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{animation::Animation, async_task::AsyncTask, close_handler::CloseHandler, global_watch::GlobalWatch, isolated_visibility::IsolatedVisibility, loadable::Loadable, popupable::Popupable, win_widget::WinWidget, SplitGenericsRef};

pub(crate) struct GeneralAttr<'a> {
    // fields about `run_after`
    pub(crate) run_after_clause: TokenStream,

    // fields about `animation`
    pub(crate) is_animation: bool,
    pub(crate) animation: Option<Animation<'a>>,
    pub(crate) animation_clause: TokenStream,
    pub(crate) animation_field: TokenStream,
    pub(crate) animation_reflect: TokenStream,
    pub(crate) animation_state_holder_field: TokenStream,
    pub(crate) animation_state_holder_impl: TokenStream,
    pub(crate) animation_state_holder_reflect: TokenStream,

    // fields about `async_task`
    pub(crate) is_async_task: bool,
    pub(crate) async_task_fields: Vec<TokenStream>,
    pub(crate) async_task_impl_clause: TokenStream,
    pub(crate) async_task_method_clause: TokenStream,

    // fields about `popupable`
    pub(crate) is_popupable: bool,
    pub(crate) popupable_field_clause: TokenStream,
    pub(crate) popupable_impl_clause: TokenStream,
    pub(crate) popupable_reflect_clause: TokenStream,

    // fields about `loadable`
    pub(crate) is_loadable: bool,
    pub(crate) loadable_field_clause: TokenStream,
    pub(crate) loadable_impl_clause: TokenStream,
    pub(crate) loadable_reflect_clause: TokenStream,

    // fields about `glboal_watch`
    pub(crate) global_watch_impl_clause: TokenStream,
    pub(crate) global_watch_reflect_clause: TokenStream,

    // fields about `iter_executor`
    pub(crate) iter_executor_reflect_clause: TokenStream,

    // fields about `frame_animator`
    pub(crate) frame_animator_reflect_clause: TokenStream,

    // fields about `isolated_visibility`
    pub(crate) is_isolated_visibility: bool,
    pub(crate) isolated_visibility_field_clause: Vec<TokenStream>,
    pub(crate) isolated_visibility_impl_clause: TokenStream,
    pub(crate) isolated_visibility_reflect_clause: TokenStream,

    // fields about `close_handler`
    pub(crate) close_handler_impl_clause: TokenStream,
    pub(crate) close_handler_reflect_clause: TokenStream,
    pub(crate) close_handler_register_clause: TokenStream,

    // fields about `win_widget`
    pub(crate) is_win_widget: bool,
    pub(crate) win_widget_field_clause: TokenStream,
    pub(crate) win_widget_reflect_clause: TokenStream,
    pub(crate) win_widget_impl_clause: TokenStream,
}

impl<'a> GeneralAttr<'a> {
    pub(crate) fn parse(ast: &DeriveInput, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        let name = &ast.ident;
        let (_, ty_generics, _) = generics;

        let mut is_run_after = false;

        let mut is_animation = false;
        let mut animation = None;

        let mut is_async_task = false;
        let mut async_tasks = vec![];

        let mut popupable = None;

        let mut loadable = None;

        let mut global_watch = None;

        let mut iter_executor = false;

        let mut frame_animator = false;

        let mut isolated_visibility = None;

        let mut close_handler = None;

        let mut win_widget = None;

        for attr in ast.attrs.iter() {
            if let Some(attr_ident) = attr.path.get_ident() {
                let attr_str = attr_ident.to_string();

                match attr_str.as_str() {
                    "run_after" => is_run_after = true,
                    "animatable" => {
                        animation = {
                            is_animation = true;
                            Some(Animation::parse(attr, generics)?)
                        }
                    }
                    "async_task" => {
                        is_async_task = true;
                        async_tasks.push(AsyncTask::parse_attr(attr, generics));
                    }
                    "popupable" => popupable = Some(Popupable::parse(ast, generics)?),
                    "loadable" => loadable = Some(Loadable::parse(ast, generics)?),
                    "global_watch" => {
                        let mut gw = attr.parse_args::<GlobalWatch>()?;
                        gw.set_generics(generics);
                        global_watch = Some(gw);
                    },
                    "iter_executor" => iter_executor = true,
                    "frame_animator" => frame_animator = true,
                    "isolated_visibility" => isolated_visibility = Some(IsolatedVisibility::parse(ast, generics)?),
                    "close_handler" => close_handler = Some(CloseHandler::parse(ast, generics)?),
                    "win_widget" => win_widget = Some(WinWidget::parse(ast, generics)?),
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
                animation: AnimationModel
            )
        } else {
            proc_macro2::TokenStream::new()
        };
        let animation_reflect = if let Some(animation) = animation.as_ref() {
            animation.animation_reflect(name)?
        } else {
            proc_macro2::TokenStream::new()
        };
        let (
            animation_state_holder_field,
            animation_state_holder_impl,
            animation_state_holder_reflect,
        ) = if let Some(animation) = animation.as_ref() {
            animation.animation_state_holder(name)?
        } else {
            (TokenStream::new(), TokenStream::new(), TokenStream::new())
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
                    #field: Option<Box<#task_name #ty_generics>>
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

        // Popupable
        let popupable_field_clause = if let Some(popupable) = popupable.as_ref() {
            popupable.popupable_field()
        } else {
            proc_macro2::TokenStream::new()
        };

        let popupable_impl_clause = if let Some(popupable) = popupable.as_ref() {
            popupable.popupable_impl()
        } else {
            proc_macro2::TokenStream::new()
        };

        let popupable_reflect_clause = if let Some(popupable) = popupable.as_ref() {
            popupable.popupable_reflect()
        } else {
            proc_macro2::TokenStream::new()
        };

        // Loadable 
        let loadable_field_clause = if let Some(loadable) = loadable.as_ref() {
            loadable.loadable_field()
        } else {
            proc_macro2::TokenStream::new()
        };

        let loadable_impl_clause = if let Some(loadable) = loadable.as_ref() {
            loadable.loadable_impl()
        } else {
            proc_macro2::TokenStream::new()
        };

        let loadable_reflect_clause = if let Some(loadable) = loadable.as_ref() {
            loadable.loadable_reflect()
        } else {
            proc_macro2::TokenStream::new()
        };

        // Global watch
        let global_watch_impl_clause = if let Some(global_watch) = global_watch.as_ref() {
            global_watch.expand_impl(name)?
        } else {
            proc_macro2::TokenStream::new()
        };

        let global_watch_reflect_clause = if let Some(global_watch) = global_watch.as_ref() {
            global_watch.expand_reflect(name)?
        } else {
            proc_macro2::TokenStream::new()
        };

        // Iter executor
        let iter_executor_reflect_clause = if iter_executor {
            let (_, ty_generics, _) = generics;
            quote!(
                type_registry.register::<#name #ty_generics, ReflectIterExecutor>();
            )
        } else {
            proc_macro2::TokenStream::new()
        };

        // Frame animator
        let frame_animator_reflect_clause = if frame_animator {
            let (_, ty_generics, _) = generics;
            quote!(
                type_registry.register::<#name #ty_generics, ReflectFrameAnimator>();
            )
        } else {
            proc_macro2::TokenStream::new()
        };

        // IsolatedVisibility
        let isolated_visibility_field_clause = if let Some(iv) = isolated_visibility.as_ref() {
            iv.isolated_visibility_field()
        } else {
           vec![] 
        };

        let isolated_visibility_impl_clause = if let Some(iv) = isolated_visibility.as_ref() {
            iv.isolated_visibility_impl()
        } else {
            proc_macro2::TokenStream::new()
        };

        let isolated_visibility_reflect_clause = if let Some(iv) = isolated_visibility.as_ref() {
            iv.isolated_visibility_reflect()
        } else {
            proc_macro2::TokenStream::new()
        };

        // CloseHandler
        let close_handler_impl_clause = if let Some(cv) = close_handler.as_ref() {
            cv.close_handler_impl()
        } else {
            proc_macro2::TokenStream::new()
        };

        let close_handler_reflect_clause = if let Some(cv) = close_handler.as_ref() {
            cv.close_handler_reflect()
        } else {
            proc_macro2::TokenStream::new()
        };

        let close_handler_register_clause = if let Some(cv) = close_handler.as_ref() {
            cv.close_handler_register()
        } else {
            proc_macro2::TokenStream::new()
        };

        // WinWidget 
        let win_widget_field_clause = if let Some(ww) = win_widget.as_ref() {
            ww.field()
        } else {
            proc_macro2::TokenStream::new()
        };

        let win_widget_reflect_clause = if let Some(ww) = win_widget.as_ref() {
            ww.relfect_clause()
        } else {
            proc_macro2::TokenStream::new()
        };

        let win_widget_impl_clause = if let Some(ww) = win_widget.as_ref() {
            ww.impl_clause()
        } else {
            proc_macro2::TokenStream::new()
        };

        Ok(Self {
            run_after_clause,
            is_animation,
            animation,
            animation_clause,
            animation_field,
            animation_reflect,
            animation_state_holder_field,
            animation_state_holder_impl,
            animation_state_holder_reflect,
            is_async_task,
            async_task_fields,
            async_task_impl_clause,
            async_task_method_clause,
            is_popupable: popupable.is_some(),
            popupable_field_clause,
            popupable_impl_clause,
            popupable_reflect_clause,
            is_loadable: loadable.is_some(),
            loadable_field_clause,
            loadable_impl_clause,
            loadable_reflect_clause,
            global_watch_impl_clause,
            global_watch_reflect_clause,
            iter_executor_reflect_clause,
            frame_animator_reflect_clause,
            is_isolated_visibility: isolated_visibility.is_some(),
            isolated_visibility_field_clause,
            isolated_visibility_impl_clause,
            isolated_visibility_reflect_clause,
            close_handler_impl_clause,
            close_handler_reflect_clause,
            close_handler_register_clause,
            is_win_widget: win_widget.is_some(),
            win_widget_field_clause,
            win_widget_reflect_clause,
            win_widget_impl_clause,
        })
    }
}
