use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Meta, MetaList, MetaNameValue, NestedMeta};
use crate::SplitGenericsRef;

static POSITION_BASED_ANIMATIONS: [&'static str; 3] = ["Linear", "EaseIn", "EaseOut"];

pub(crate) struct Animation<'a> {
    pub(crate) mode: Option<Ident>,
    pub(crate) ty: Option<Ident>,
    pub(crate) direction: Option<Ident>,
    pub(crate) effect: Option<Ident>,
    pub(crate) duration: Option<i32>,
    attr: Attribute,
    generics: SplitGenericsRef<'a>,
}

impl<'a> Animation<'a> {
    pub fn parse(attr: &Attribute, generics: SplitGenericsRef<'a>) -> syn::Result<Self> {
        let mut animation = Self {
            mode: None,
            ty: None,
            direction: None,
            effect: None,
            duration: None,
            attr: attr.clone(),
            generics,
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
                                    "mode" => {
                                        match lit {
                                            syn::Lit::Str(lit) => {
                                                let lit_str = lit.value();
                                                animation.mode = Some(Ident::new(&lit_str, lit.span()));
                                            }
                                            _ => return Err(syn::Error::new_spanned(
                                                attr,
                                                "Proc-macro `animatable`: value of config `mode` should be literal.",
                                            )),
                                        }
                                    }
                                    "ty" => {
                                        match lit {
                                            syn::Lit::Str(lit) => {
                                                let lit_str = lit.value();
                                                animation.ty = Some(Ident::new(&lit_str, lit.span()));
                                            }
                                            _ => return Err(syn::Error::new_spanned(
                                                attr,
                                                "Proc-macro `animatable`: value of config `ty` should be literal.",
                                            )),
                                        }
                                    }
                                    "direction" => {
                                        match lit {
                                            syn::Lit::Str(lit) => {
                                                let lit_str = lit.value();
                                                animation.direction = Some(Ident::new(&lit_str, lit.span()));
                                            }
                                            _ => return Err(syn::Error::new_spanned(
                                                attr,
                                                "Proc-macro `animatable`: value of config `direction` should be literal.",
                                            )),
                                        }
                                    }
                                    "effect" => {
                                        match lit {
                                            syn::Lit::Str(lit) => {
                                                let lit_str = lit.value();
                                                animation.effect = Some(Ident::new(&lit_str, lit.span()));
                                            }
                                            _ => return Err(syn::Error::new_spanned(
                                                attr,
                                                "Proc-macro `animatable`: value of config `effect` should be literal.",
                                            )),
                                        }
                                    }
                                    "duration" => {
                                        match lit {
                                            syn::Lit::Int(lit) => {
                                                let duration = lit.base10_parse()?;
                                                animation.duration = Some(duration);
                                            }
                                            _ => return Err(syn::Error::new_spanned(
                                                attr,
                                                "Proc-macro `animatable`: value of config `duration` should be int.",
                                            )),
                                        }
                                    }
                                    _ => {
                                        return Err(syn::Error::new_spanned(
                                            attr,
                                            "Proc-macro `animatable`: only support config `ty = xxx`",
                                        ))
                                    }
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    attr,
                                    "Proc-macro `animatable`: only support config `ty = xxx, direction = xxx, duration = xxx`",
                                ))
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if animation.ty.is_none() || animation.duration.is_none() {
            return Err(syn::Error::new_spanned(
                attr,
                "Parse proc-macro `animatable` failed, attribute `ty`,`duration` can not be `None`",
            ));
        }

        let ty = animation.ty.as_ref().unwrap().to_string();
        if POSITION_BASED_ANIMATIONS.contains(&ty.as_str()) {
            if animation.direction.is_none() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "Position-based animations must assign direction.(Linear, EaseIn, EaseOut)",
                ));
            }
        } else {
            if animation.direction.is_some()
                || animation.effect.is_some()
                || animation.mode.is_some()
            {
                return Err(syn::Error::new_spanned(
                    attr,
                    "Unable to assign `direction`,`effect` and `mode` on transparency-based animations.(FadeLinear, FadeEaseIn, FadeEaseOut)",
                ));
            }
        }

        Ok(animation)
    }

    pub(crate) fn generate_animation(&self, name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        let (impl_generics, ty_generics, where_clause) = &self.generics;

        let clause = quote!(
            impl #impl_generics Animatable for #name #ty_generics #where_clause {
                #[inline]
                fn set_animation(&mut self, animation: Animation) {
                    self.animation.set_animation(animation)
                }

                #[inline]
                fn animation(&self) -> Animation {
                    self.animation.animation()
                }

                #[inline]
                fn animation_model(&self) -> &AnimationModel {
                    &self.animation
                }

                #[inline]
                fn animation_model_mut(&mut self) -> &mut AnimationModel {
                    &mut self.animation
                }
            }

            impl #impl_generics Snapshot for #name #ty_generics #where_clause {
                #[inline]
                fn as_snapshot(&self) -> &dyn Snapshot {
                    self
                }

                #[inline]
                fn as_snapshot_mut(&mut self) -> &mut dyn Snapshot {
                    self
                }

                #[inline]
                fn as_widget(&self) -> &dyn WidgetImpl {
                    self
                }

                #[inline]
                fn as_widget_mut(&mut self) -> &mut dyn WidgetImpl {
                    self
                }
            }
        );

        Ok(clause)
    }

    pub(crate) fn parse_default(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mode = if let Some(mode) = self.mode.as_ref() {
            format!("animation::AnimationMode::{}", mode.to_string())
        } else {
            "Default::default()".to_string()
        };
        let effect = if let Some(effect) = self.effect.as_ref() {
            format!("animation::AnimationEffect::{}", effect.to_string())
        } else {
            "Default::default()".to_string()
        };
        let direction = if let Some(direction) = self.direction.as_ref() {
            format!("Some(animation::Direction::{})", direction.to_string())
        } else {
            "None".to_string()
        };
        let ty = self.ty.as_ref().unwrap();
        let duration = *self.duration.as_ref().unwrap();

        let default = format!("animation::AnimationModel::new({}, animation::Animation::{}, std::time::Duration::from_millis({}), {}, Some({}))", mode, ty.to_string(), duration, direction, effect);

        Ok(quote!(
            #[derivative(Default(value = #default))]
        ))
    }

    pub(crate) fn animation_reflect(&self, name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        let (_, ty_generics, _) = &self.generics;
        Ok(quote!(
            type_registry.register::<#name #ty_generics, ReflectSnapshot>();
        ))
    }

    pub(crate) fn animation_state_holder(
        &self,
        name: &Ident,
    ) -> syn::Result<(TokenStream, TokenStream, TokenStream)> {
        match self.ty.as_ref() {
            Some(ty) => match ty.to_string().as_str() {
                "Linear" => rect_holder(name, self.generics),
                "EaseIn" => rect_holder(name, self.generics),
                "EaseOut" => rect_holder(name, self.generics),
                "FadeLinear" => transparency_holder(name, self.generics),
                "FadeEaseIn" => transparency_holder(name, self.generics),
                "FadeEaseOut" => transparency_holder(name, self.generics),
                str => Err(syn::Error::new_spanned(
                    self.attr.clone(),
                    format!("Unexpected animation type: {}", str),
                )),
            },
            None => Err(syn::Error::new_spanned(
                self.attr.clone(),
                "Animation type was None.",
            )),
        }
    }
}

fn rect_holder<'a>(
    name: &Ident,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'a>,
) -> syn::Result<(TokenStream, TokenStream, TokenStream)> {
    Ok((
        quote!(
            animated_rect: Box<Rect>
        ),
        quote!(
            impl #impl_generics RectHolder for #name #ty_generics #where_clause {
                #[inline]
                fn animated_rect(&self) -> Rect {
                    *self.animated_rect.as_ref()
                }

                #[inline]
                fn animated_rect_mut(&mut self) -> &mut Rect {
                    self.animated_rect.as_mut()
                }
            }
        ),
        quote!(
            type_registry.register::<#name #ty_generics, ReflectRectHolder>();
        ),
    ))
}

fn transparency_holder<'a>(
    name: &Ident,
    (impl_generics, ty_generics, where_clause): SplitGenericsRef<'a>,
) -> syn::Result<(TokenStream, TokenStream, TokenStream)> {
    Ok((
        quote!(
            animated_transparency: Box<i32>
        ),
        quote!(
            impl #impl_generics TransparencyHolder for #name #ty_generics #where_clause {
                #[inline]
                fn animated_transparency(&self) -> i32 {
                    *self.animated_transparency.as_ref()
                }

                #[inline]
                fn animated_transparency_mut(&mut self) -> &mut i32 {
                    self.animated_transparency.as_mut()
                }
            }
        ),
        quote!(
            type_registry.register::<#name #ty_generics, ReflectTransparencyHolder>();
        ),
    ))
}
