use proc_macro2::Ident;
use quote::quote;

pub(crate) fn generate_animation(name: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    let clause = quote!(
        impl Animatable for #name {
            #[inline]
            fn set_animation(&mut self, animation: Animations) {
                self.animation.set_animation(animation)
            }

            #[inline]
            fn animation(&self) -> Animations {
                self.animation.animation()
            }
        }
    );

    Ok(clause)
}