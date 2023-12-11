use quote::quote;
use syn::{parse::Parse, Ident, Token};

pub(crate) struct AsyncDoParser {
    has_move: bool,
    body: proc_macro2::TokenStream,
    then: Option<proc_macro2::TokenStream>,
    parameter: Option<Ident>,
}

impl Parse for AsyncDoParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let move_clause = input.parse::<Option<Token!(move)>>()?;

        let body_buf;
        syn::braced!(body_buf in input);
        let mut body: proc_macro2::TokenStream = body_buf.parse()?;
        body.extend(quote!(.to_value()));

        let fat_arrow = input.parse::<Option<Token!(=>)>>()?;
        let mut then = None;
        let mut parameter = None;
        if fat_arrow.is_some() {
            input.parse::<Option<Token!(|)>>()?;
            let parameter_ident: Option<Ident> = input.parse()?;
            input.parse::<Option<Token!(|)>>()?;

            let then_buf;
            syn::braced!(then_buf in input);
            let then_clause: proc_macro2::TokenStream = then_buf.parse()?;
            then = Some(then_clause);
            parameter = parameter_ident;
        }

        Ok(AsyncDoParser {
            has_move: move_clause.is_some(),
            body,
            then,
            parameter,
        })
    }
}

impl AsyncDoParser {
    pub fn expand(mut self) -> proc_macro2::TokenStream {
        let body = &self.body;

        let mut move_clause = proc_macro2::TokenStream::new();
        if self.has_move {
            move_clause.extend(quote!(move));
        }

        let mut then_clause = proc_macro2::TokenStream::new();
        if self.then.is_some() {
            let then_body = self.then.as_ref().unwrap();
            let mut param_clause = proc_macro2::TokenStream::new();
            if let Some(param) = self.parameter.take() {
                param_clause.extend(quote!(#param))
            } else {
                param_clause.extend(quote!(_))
            };
            then_clause.extend(quote!(
                .then(move |#param_clause| {
                    #then_body
                })
            ))
        }

        quote!(
            {
                let join_handler = tokio::spawn(async #move_clause { #body });
                let task = AsyncTask::new(join_handler)#then_clause;
                async_tasks().entry(std::thread::current().id()).or_insert(vec![]).push(task);
            }
        )
    }
}
