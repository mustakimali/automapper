use std::path::PathBuf;

use anyhow::Context;
use serde_json::Value;

use crate::rdoc_parser;

use super::{FqIdent, RustType};

pub struct PathCache {
    path: Vec<FqIdent>,
}

pub struct Cache {
    pub types: Vec<RustType>,
    pub paths: PathCache,
    pub rustdoc_json: Value,
}

impl Cache {
    pub fn new_from_rust_doc(rustdoc_path: PathBuf) -> anyhow::Result<Self> {
        let rustdoc_json: Value = serde_json::from_str(
            &std::fs::read_to_string(&rustdoc_path).expect("failed to read rustdoc.json"),
        )
        .context("failed to parse rustdoc.json")?;

        Self::new_from_rust_doc_json(rustdoc_json)
    }

    pub fn new_from_rust_doc_json(rustdoc_json: Value) -> anyhow::Result<Self> {
        let path_cache = rdoc_parser::find_all_rusttype_fq_path(&rustdoc_json)
            .context("failed to find all struct and fq path")?
            .into_iter()
            .collect::<Vec<_>>();

        let paths = PathCache::new(path_cache);

        let types = rdoc_parser::enumerate_rust_types(&rustdoc_json, &paths)
            .context("failed to enumerate rust types")?
            .collect::<Vec<_>>();

        Ok(Self {
            types,
            paths,
            rustdoc_json,
        })
    }

    pub fn find(&self, ident: &FqIdent) -> Option<&RustType> {
        let fq_path = self.paths.find(ident.name()).map(|f| f.crate_scoped());

        self.types.iter().find(|ty| match &fq_path {
            Some(fq) => ty.equals_fq_path(fq),
            //None => ty.name().ends_with(&ident.name_string()),
            None => {
                let e = format!("failed to find fq path for {:?}", ident);
                dbg!(e);
                false
            }
        })
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

    pub fn find_fully_qualified_path(&self, ident: &FqIdent) -> Option<FqIdent> {
        self.find(ident.name()).map(|f| f.crate_scoped())
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

    #[test]
    fn find_nested_fields() {
        let rustdoc = rdoc_parser::test::get_test_data();
        let cache = Cache::new_from_rust_doc_json(rustdoc).unwrap();

        let found = cache
            .find(&FqIdent::try_from_str("nested_inner::NestedSourceInnerType").unwrap())
            .expect("find nested type");
    }
}
