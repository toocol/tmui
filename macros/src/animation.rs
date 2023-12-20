use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Meta, MetaList, MetaNameValue, NestedMeta};

pub(crate) struct Animation {
    pub(crate) ty: Option<Ident>,
    pub(crate) direction: Option<Ident>,
    pub(crate) duration: Option<i32>,
    attr: Attribute,
}

impl Animation {
    pub fn parse(attr: &Attribute) -> syn::Result<Self> {
        let mut animation = Self {
            ty: None,
            direction: None,
            duration: None,
            attr: attr.clone(),
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

        if animation.ty.is_none() || animation.direction.is_none() || animation.duration.is_none() {
            return Err(syn::Error::new_spanned(
                attr,
                "Parse proc-macro `animatable` failed, only support config `ty = xxx, direction = xxx, duration = xxx`",
            ));
        }

        Ok(animation)
    }

    pub(crate) fn generate_animation(&self, name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        let clause = quote!(
            impl Animatable for #name {
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

            impl Snapshot for #name {
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
        let ty = self.ty.as_ref().unwrap();
        let direction = self.direction.as_ref().unwrap();
        let duration = *self.duration.as_ref().unwrap();

        let default = format!("animation::AnimationModel::new(animation::Animation::{}, animation::Direction::{}, std::time::Duration::from_millis({}))", ty.to_string(), direction.to_string(), duration);

        Ok(quote!(
            #[derivative(Default(value = #default))]
        ))
    }

    pub(crate) fn animation_reflect(&self, name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote!(
            type_registry.register::<#name, ReflectSnapshot>();
        ))
    }

    pub(crate) fn animation_state_holder(
        &self,
        name: &Ident,
    ) -> syn::Result<(TokenStream, TokenStream, TokenStream)> {
        match self.ty.as_ref() {
            Some(ty) => match ty.to_string().as_str() {
                "Linear" => rect_holder(name),
                "EaseIn" => rect_holder(name),
                "EaseOut" => rect_holder(name),
                "FadeLinear" => color_holder(name),
                "FadeEaseIn" => color_holder(name),
                "FadeEaseOut" => color_holder(name),
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

fn rect_holder(name: &Ident) -> syn::Result<(TokenStream, TokenStream, TokenStream)> {
    Ok((
        quote!(
            animated_rect: Box<Rect>
        ),
        quote!(
            impl RectHolder for #name {
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
            type_registry.register::<#name, ReflectRectHolder>();
        ),
    ))
}

fn color_holder(name: &Ident) -> syn::Result<(TokenStream, TokenStream, TokenStream)> {
    Ok((
        quote!(
            animated_color: Box<Color>
        ),
        quote!(
            impl ColorHolder for #name {
                #[inline]
                fn animated_color(&self) -> Color {
                    *self.animated_color.as_ref()
                }

                #[inline]
                fn animated_color_mut(&mut self) -> &mut Color {
                    self.animated_color.as_mut()
                }
            }
        ),
        quote!(
            type_registry.register::<#name, ReflectColorHolder>();
        ),
    ))
}
