use std::path::PathBuf;

use anyhow::Context;
use serde_json::Value;

use crate::rustdoc_json_parser;

use super::{FqIdent, RustType};

#[derive(Debug)]
pub struct PathCache {
    path: Vec<FqIdent>,
}

#[derive(Debug)]
pub struct Cache {
    types: Vec<RustType>,
    pub paths: PathCache,
    pub rustdoc_json: Value,
}

impl Cache {
    pub fn new_from_rust_doc(rustdoc_path: PathBuf) -> anyhow::Result<Self> {
        let rustdoc_json: Value = serde_json::from_str(
            &std::fs::read_to_string(&rustdoc_path).expect("failed to read rustdoc.json"),
        )
        .context("failed to parse rustdoc.json")?;

        let path_cache = rustdoc_json_parser::find_all_rusttype_fq_path(&rustdoc_json)
            .context("failed to find all struct and fq path")?
            .into_iter()
            .collect::<Vec<_>>();

        let types = rustdoc_json_parser::enumerate_rust_types(&rustdoc_json)
            .context("failed to enumerate rust types")?
            .collect::<Vec<_>>();
        let paths = PathCache::new(path_cache);

        Ok(Self {
            types,
            paths,
            rustdoc_json,
        })
    }

    pub fn find(&self, ident: &FqIdent) -> Option<&RustType> {
        self.types.iter().find(|i| i.name() == ident.to_string())
    }
}

impl PathCache {
    pub fn new(idents: Vec<FqIdent>) -> Self {
        // todo: build fast lookup cache

        Self { path: idents }
    }

    pub fn find(&self, name: &[syn::Ident]) -> Option<&FqIdent> {
        self.path.iter().find(|i| i.path.ends_with(name))
    }

    pub fn find_by_path(&self, name: &syn::Path) -> Option<&FqIdent> {
        let seg = name
            .segments
            .iter()
            .map(|s| s.ident.clone())
            .collect::<Vec<_>>();

        self.find(&seg)
    }
}

#[cfg(test)]
mod test {
    use quote::format_ident;

    use super::*;

    #[test]
    fn search() {
        let cache = PathCache::new(vec![FqIdent::from_idents(vec![
            format_ident!("std"),
            format_ident!("path"),
            format_ident!("Path"),
        ])]);

        assert!(cache.find(&[format_ident!("Path")]).is_some());
        //assert!(cache.find_by_path(syn::Path::parse).is_some()) // todo: test
    }
}
