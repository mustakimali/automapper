use std::path::PathBuf;

use crate::rustdoc_json_parser::models::{Struct, StructField};
use anyhow::Context;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde_json::Value;
use syn::{
    braced, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated, token,
    DeriveInput, Meta, Token,
};
use walkdir::WalkDir;

mod rustdoc_json_parser;

struct TraitImpl {
    struct_token: Token![fn],
    iden: syn::Ident,
    paren_token: token::Paren,
    mapping: Request,
    semi_token: Token![;],
}

#[derive(Clone)]
struct Request {
    source_type: syn::Ident,
    _coma: syn::Token![,],
    dest_type: syn::Ident,
}

#[proc_macro]
pub fn lazy_map(input: TokenStream) -> TokenStream {
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
        let rustdoc_path = cargo_toml_path.parent().unwrap().join("rustdoc.json");

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

        let rustdoc_json: Value = serde_json::from_str(
            &std::fs::read_to_string(&rustdoc_path).expect("failed to read rustdoc.json"),
        )
        .expect("failed to parse rustdoc.json");

        let root = Mapping::new(
            format_ident!("value"),
            self.mapping.source_type.clone(),
            self.mapping.dest_type.clone(),
            &rustdoc_json,
        )
        .with_context(|| {
            format!(
                "failed to find root struct {} and resolve fields",
                self.mapping.source_type.to_string()
            )
        })
        .unwrap();

        let source_ty_name = self.mapping.source_type.clone();
        let dest_ty_name = self.mapping.dest_type.clone();
        let method_name = self.iden.clone();
        let method_name_rev = format_ident!("{}_rev", self.iden.clone());
        tokens.extend(quote! {
            fn #method_name(value: #source_ty_name) -> #dest_ty_name {
                #root
            }
        });
    }
}

/// Generates mapping of a struct
/// ```no_run
/// MyType {
///     field1: value.field1,
///     field2: value.field2,
/// }
/// ```
struct Mapping<'v> {
    source_field_name: syn::Ident,
    source_type: syn::Ident,
    dest_type: syn::Ident,
    rustdoc_json: &'v Value,
    source_struct: Struct,
    source_fields: Vec<StructField>,
    dest_struct: Struct,
    dest_fields: Vec<StructField>,
}

impl<'v> Mapping<'v> {
    pub fn new(
        source_field_name: syn::Ident,
        source_type: syn::Ident,
        dest_type: syn::Ident,
        rustdoc_json: &'v Value,
    ) -> anyhow::Result<Self> {
        let (source_struct, source_fields) =
            rustdoc_json_parser::find_struct_and_resolve_fields_for_ident(
                &source_type,
                &rustdoc_json,
            )
            .with_context(|| {
                format!(
                    "failed to find source struct {} and resolve fields",
                    source_type.to_string()
                )
            })?;
        let (dest_struct, dest_fields) =
            rustdoc_json_parser::find_struct_and_resolve_fields_for_ident(
                &dest_type,
                &rustdoc_json,
            )
            .with_context(|| {
                format!(
                    "failed to find dest struct {} and resolve fields",
                    dest_type.to_string()
                )
            })?;

        Ok(Self {
            source_field_name,
            source_type,
            dest_type,
            rustdoc_json,
            source_struct,
            source_fields,
            dest_struct,
            dest_fields,
        })
    }
}

impl<'v> ToTokens for Mapping<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let assignments = self
            .dest_fields
            .iter()
            .map(|f| Assignment {
                field: f.name.clone(),
                value_field_name: self.source_field_name.clone(),
                value: "todo!()".to_string(),
            })
            .collect::<Vec<_>>();

        let source_ty_name = self.source_type.clone();
        let dest_ty_name = self.dest_type.clone();
        tokens.extend(quote! {
            #dest_ty_name {
                #(#assignments)*
            }

        });
    }
}

struct Assignment {
    field: String,
    value_field_name: syn::Ident,
    value: String,
}

impl ToTokens for Assignment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = format_ident!("{}", self.field.clone());
        let value_field = format_ident!("{}", self.value_field_name.clone());
        tokens.extend(quote! {
            #name: #value_field.#name,
        });
    }
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
        let Ok(cargo_toml) = std::fs::read_to_string(&entry.path()) else {
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
