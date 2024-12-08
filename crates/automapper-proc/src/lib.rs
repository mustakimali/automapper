#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    collections::HashSet, io::Write, ops::Deref, path::PathBuf, sync::Arc, time::SystemTime,
};

use anyhow::Context;
use mapper::TypeToTypeMapping;
use models::context::MacroCtx;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use rodc_util::StructRustType;
use serde_json::Value;
use syn::{
    braced, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated, token,
    DeriveInput, Meta, Token,
};
use ulid::Ulid;
use walkdir::WalkDir;

mod mapper;
mod models;
mod rodc_util;

#[derive(Debug)]
struct TraitImpl {
    struct_token: Token![fn],
    iden: syn::Ident,
    paren_token: token::Paren,
    //mapping: Request,
    source_type: syn::Path,
    arrow_token: Token![->],
    dest_type: syn::Path,
    expr_token: Option<syn::Expr>,
    semi_token: Option<Token![;]>,
}

/// See crate level doc for automapper for more information.
#[proc_macro]
pub fn impl_map_fn(input: TokenStream) -> TokenStream {
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
            source_type: content.parse()?,
            arrow_token: input.parse()?,
            dest_type: input.parse()?,
            expr_token: input.parse().ok(),
            semi_token: input.parse()?,
        };

        Ok(this)
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

        let rdocs = {
            let content =
                std::fs::read_to_string(rustdoc_path).expect("read rustdoc.json file from disk");
            serde_json::from_str::<rustdoc_types::Crate>(&content)
                .expect("parse rustdoc.json as json")
        };

        let ctx = MacroCtx::new(rdocs);

        let mapping = TypeToTypeMapping::new(
            self.source_type.clone(),
            vec!["value".to_string()], // the name of the input variable in the mapping function
            self.dest_type.clone(),
            ctx,
        )
        .expect("create struct to struct mapping");

        let fn_name = self.iden.clone();
        let value_ty = mapping.source.path();
        let dest_ty = mapping.dest.path();

        let t = quote! {
            fn #fn_name(value: #value_ty) -> #dest_ty {
                #mapping
            }
        };

        #[cfg(debug_assertions)]
        write_debug(t.to_string());

        tokens.extend(t);
    }
}

fn write_debug(code: String) {
    use std::path::Path;
    let output_rs_file = Path::new("crates/usage/src/output.rs");
    if output_rs_file.exists() {
        let modified = output_rs_file
            .metadata()
            .expect("metadata")
            .modified()
            .expect("modified");
        if modified.elapsed().expect("elapsed").as_secs() > 2 {
            _ = std::fs::remove_file(output_rs_file);
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_rs_file)
        .expect("open file");

    let mod_name = Ulid::new().to_string()[12..].to_lowercase();
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time")
        .as_millis();
    let code = format!(
        r#"

        pub mod mod_{}_{} {{
            {}
        }}"#,
        time, mod_name, code
    );
    file.write_all(code.as_bytes()).expect("write to file");

    //std::fs::write("crates/usage/src/output.rs", t.to_string()).expect("write to output.rs");
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
