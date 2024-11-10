use std::{hash::Hash, path::Display};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

pub use cache::{Cache, PathCache};
pub use ctx::{MacroContextInner, MacroCtx};
pub use fq_ident::FqIdent;

mod cache;
mod ctx;
mod fq_ident;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    Struct {
        item: Struct,
        fq_path: Option<FqIdent>,
        fields: Vec<StructField>,
    },
    Enum {
        item: Enum,
        fq_path: Option<FqIdent>,
        variants: Vec<EnumVariant>,
    },
}

impl RustType {
    pub fn name(&self) -> &str {
        match self {
            RustType::Struct { item, .. } => &item.name,
            RustType::Enum { item, .. } => &item.name,
        }
    }
    pub fn fully_qualified_path(&self) -> &Option<FqIdent> {
        match self {
            RustType::Struct { fq_path, .. } => fq_path,
            RustType::Enum { fq_path, .. } => fq_path,
        }
    }

    pub fn equals_fq_path(&self, ident: &FqIdent) -> bool {
        self.fully_qualified_path()
            .as_ref()
            .map(|p| p == ident)
            .unwrap_or_default()
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, RustType::Struct { .. })
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, RustType::Enum { .. })
    }

    pub fn same_kind(&self, other: &Self) -> bool {
        match (self, other) {
            (RustType::Struct { .. }, RustType::Struct { .. }) => true,
            (RustType::Enum { .. }, RustType::Enum { .. }) => true,
            _ => false,
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            RustType::Struct { .. } => "struct",
            RustType::Enum { .. } => "enum",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub name: String,
    pub field_ids: Vec<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub variant_ids: Vec<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub id: Id,
    pub name: String,
    pub ty: StructFieldKind,
    pub external_crate_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant {
    pub id: Id,
    pub name: String,
    pub ty: EnumVariantKind,
}

impl Hash for EnumVariant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Hash for StructField {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// ComplexEnumSource: 439 (enum)
// Variants: 488, 490, 492
// 488 - plain
// 490 - struct
// 492 - toupe

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum EnumVariantKind {
    Plain {
        discriminant: Option<String>,
    },
    Struct {
        fields: Vec<u32>,
        has_stripped_fields: bool,
    },
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
            StructFieldKind::Primitive(n) => FqIdent::try_from_str(&n).expect("n"),
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