#![allow(dead_code)]
#![allow(unused_imports)]

use std::{collections::HashSet, path::PathBuf, sync::Arc};

use anyhow::Context;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde_json::Value;
use syn::{
    braced, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated, token,
    DeriveInput, Meta, Token,
};
use walkdir::WalkDir;

mod rodc_util;

#[derive(Debug)]
struct TraitImpl {
    struct_token: Token![fn],
    iden: syn::Ident,
    paren_token: token::Paren,
    mapping: Request,
    semi_token: Token![;],
}

#[derive(Debug, Clone)]
struct Request {
    source_type: syn::Path,
    _coma: syn::Token![,],
    dest_type: syn::Path,
}

#[proc_macro]
pub fn map(input: TokenStream) -> TokenStream {
    let def = parse_macro_input!(input as TraitImpl);
    def.into_token_stream().into()
}

impl Parse for TraitImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let this = Self {
            struct_token: input.parse()?,
            iden: input.parse()?,
            paren_token: parenthesized!(content in input),
            mapping: content.parse()?,
            semi_token: input.parse()?,
        };

        Ok(this)
    }
}

impl Parse for Request {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            source_type: input.parse()?,
            _coma: input.parse()?,
            dest_type: input.parse()?,
        })
    }
}

impl ToTokens for TraitImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let cargo_toml_path = caller_crate_cargo_toml();
        let rustdoc_path = cargo_toml_path.parent().unwrap().join("rustdoc_v2.json");

        if !rustdoc_path.exists() {
            eprintln!(
                "rustdoc.json does not exist at {:?}, run the cli generate this.",
                rustdoc_path
            );
            tokens.extend(quote! {
                panic!("rustdoc.json does not exist at {:?}, run the cli generate this.", rustdoc_path);
            });
            return;
        };

        let rdocs = {
            let content =
                std::fs::read_to_string(rustdoc_path).expect("read rustdoc.json file from disk");
            serde_json::from_str::<rustdoc_types::Crate>(&content)
                .expect("parse rustdoc.json as json")
        };

        let source = rodc_util::find_struct_by_exact_name(self.mapping.source_type.clone(), &rdocs)
            .with_context(|| {
                format!(
                    "failed to find source struct: {}",
                    self.mapping.source_type.to_token_stream().to_string()
                )
            })
            .unwrap();

        let dest = rodc_util::find_struct_by_exact_name(self.mapping.dest_type.clone(), &rdocs)
            .with_context(|| {
                format!(
                    "failed to find dest struct: {}",
                    self.mapping.dest_type.to_token_stream().to_string()
                )
            })
            .unwrap();

        dbg!(&source, &dest);
        unimplemented!()
    }
}

struct StructMapping {
    source_path: syn::Path,
    dest_path: syn::Path,
}

/// Returns the root path of the crate that calls this function.
/// This is a cursed method
fn caller_crate_cargo_toml() -> PathBuf {
    let crate_name =
        std::env::var("CARGO_PKG_NAME").expect("failed to read ENV var `CARGO_PKG_NAME`!");
    let current_dir = std::env::current_dir().expect("failed to unwrap env::current_dir()!");
    let search_entry = format!("name=\"{crate_name}\"");
    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(|e| !e.file_name().eq_ignore_ascii_case("target"))
    {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(file_name) = entry.path().file_name() else {
            continue;
        };
        if !file_name.eq_ignore_ascii_case("Cargo.toml") {
            continue;
        }
        let Ok(cargo_toml) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        if cargo_toml
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>()
            .contains(search_entry.as_str())
        {
            return entry.path().to_path_buf();
        }
    }
    current_dir
}
