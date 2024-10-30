use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated, token,
    DeriveInput, Meta, Token,
};

mod rustdoc_json;

struct TraitImpl {
    struct_token: Token![fn],
    iden: syn::Ident,
    paren_token: token::Paren,
    fields: Punctuated<Request, Token![,]>,
    semi_token: Token![;],
}

#[derive(Clone)]
struct Request {
    name: syn::Ident,
    _colon: syn::Token![,],
    input: syn::Ident,
}

#[proc_macro]
pub fn lazy_map(input: TokenStream) -> TokenStream {
    let def = parse_macro_input!(input as TraitImpl);
    def.into_token_stream().into()
}

impl Parse for TraitImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        dbg!(&input);
        let content;
        let this = Self {
            struct_token: input.parse()?,
            iden: input.parse()?,
            paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(Request::parse, Token![,])?,
            semi_token: input.parse()?,
        };
        if this.fields.len() != 1 {
            panic!("Expected two types only");
        }

        Ok(this)
    }
}

impl Parse for Request {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            input: input.parse()?,
        })
    }
}

impl ToTokens for TraitImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {})
    }
}
