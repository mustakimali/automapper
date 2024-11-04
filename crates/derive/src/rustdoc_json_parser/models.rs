use std::path::Display;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

pub use ctx::{MacroCtx, MacroContextInner};
pub use fq_ident::FqIdent;
pub use path_dict::PathCache;

mod ctx;
mod fq_ident;
mod path_dict;

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub field_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub id: String,
    pub name: String,
    pub ty: StructFieldKind,
    pub external_crate_name: Option<String>,
}

impl StructField {
    pub fn is_primitive(&self) -> bool {
        matches!(self.ty, StructFieldKind::Primitive(_))
    }
    pub fn type_name(&self) -> FqIdent {
        self.ty.name_ident()
    }
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum StructFieldKind {
    Primitive(String),
    ResolvedPath(ResolvedPathStructField),
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedPathStructField {
    pub id: u32,
    pub name: String,
}

pub trait ResolvedPathExt {
    fn name_ident(&self) -> FqIdent;
}

impl ResolvedPathExt for ResolvedPathStructField {
    fn name_ident(&self) -> FqIdent {
        let path = self
            .name
            .split("::")
            .map(|s| format_ident!("{}", s))
            .collect();

        FqIdent::from_idents(path)
    }
}

impl ResolvedPathExt for StructFieldKind {
    fn name_ident(&self) -> FqIdent {
        match self {
            StructFieldKind::Primitive(n) => FqIdent::try_from(&n).expect("n"),
            StructFieldKind::ResolvedPath(path) => path.name_ident(),
        }
    }
}

impl ResolvedPathExt for &Struct {
    fn name_ident(&self) -> FqIdent {
        let path = self
            .name
            .split("::")
            .map(|s| format_ident!("{}", s))
            .collect();

        FqIdent::from_idents(path)
    }
}
