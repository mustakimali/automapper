use anyhow::{anyhow, bail, Context};
use quote::{format_ident, ToTokens};
use rustdoc_types::{Crate, GenericArg, GenericArgs, Item, ItemSummary};
use search::SearchResult;

mod search;

//
// Public
//

pub fn query_types(name: &syn::Path, rdocs: &Crate) -> anyhow::Result<Vec<RustType>> {
    let mut structs = query_structs(name, rdocs)?
        .into_iter()
        .map(RustType::Struct)
        .collect::<Vec<_>>();

    let enums = query_enums(name, rdocs)?.into_iter().map(RustType::Enum);
    structs.extend(enums.collect::<Vec<_>>());

    Ok(structs)
}

/// Find a type (struct/enum) by name,
/// return the item with exact match or a similar one otherwise.
pub fn find_types_try_exact(name: &syn::Path, rdocs: &Crate) -> anyhow::Result<RustType> {
    let items = query_types(name, rdocs)?;
    items
        .iter()
        .find(|i| i.exact_match())
        .or_else(|| items.first())
        .cloned()
        .with_context(|| format!("failed to find type: {}", name.to_token_stream()))
}

pub fn find_path_by_id(id: &rustdoc_types::Id, rdocs: &Crate) -> syn::Path {
    let dc = rdocs.paths.get(id).unwrap();

    if !matches!(
        dc.kind,
        rustdoc_types::ItemKind::Struct | rustdoc_types::ItemKind::Enum
    ) {
        unreachable!("find_path_by_id: path must be a struct or enum type")
    }

    syn::parse_str(&dc.path.join("::")).expect("failed to parse path")
}

//
// Private
//

/// Find enums by name.
///
///
fn query_enums(name: &syn::Path, rdocs: &Crate) -> anyhow::Result<Vec<EnumRustType>> {
    search::query_items(name, rdocs)
        .context("find struct by name")?
        .into_iter()
        .filter(|i| matches!(i.item.kind, rustdoc_types::ItemKind::Enum))
        .map(|result| _find_enum_with_resolved_variants(&result, rdocs))
        .collect::<anyhow::Result<Vec<_>>>()
}

/// Find structs by name.
///
/// This function will return a list of structs that match the given name partially or exactly.
/// Check the [StructWrapper::is_exact_match] field to see if the match was exact or not.
///
fn query_structs(name: &syn::Path, rdocs: &Crate) -> anyhow::Result<Vec<StructRustType>> {
    search::query_items(name, rdocs)
        .context("find struct by name")?
        .into_iter()
        .filter(|i| matches!(i.item.kind, rustdoc_types::ItemKind::Struct))
        .map(|result| _find_struct_with_resolved_fields(&result, rdocs))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn _find_enum_with_resolved_variants(
    result: &SearchResult,
    rdocs: &Crate,
) -> Result<EnumRustType, anyhow::Error> {
    let rustdoc_types::ItemKind::Enum = result.item.kind else {
        bail!("not an enum type")
    };

    let (_, item) = rdocs
        .index
        .iter()
        .find(|(_, item)| item.id.eq(result.id))
        .context("locate struct in .index")?;

    let rustdoc_types::ItemEnum::Enum(enum_) = &item.inner else {
        unreachable!("_find_enum_with_resolved_variants: must be a struct type",)
    };

    let variants = enum_
        .variants
        .iter()
        .flat_map(|v| rdocs.index.get(v))
        .map(|e| {
            let rustdoc_types::ItemEnum::Variant(ref variant) = e.inner else {
                unreachable!("variant must be a variant type")
            };

            match &variant.kind {
                rustdoc_types::VariantKind::Plain => EnumVariant {
                    name: e.name.clone().expect("name of enum variant"),
                    kind: EnumVariantKind::Plain,
                },
                rustdoc_types::VariantKind::Tuple(items) => EnumVariant {
                    name: e.name.clone().expect("name of enum variant"),
                    kind: EnumVariantKind::Tuple(_resolve_fields(
                        rdocs,
                        items.iter().flatten().copied().collect::<Vec<_>>().as_ref(),
                    )),
                },
                rustdoc_types::VariantKind::Struct {
                    fields,
                    has_stripped_fields: _,
                } => EnumVariant {
                    name: e.name.clone().expect("name of enum variant"),
                    kind: EnumVariantKind::Struct {
                        fields: _resolve_fields(rdocs, fields),
                    },
                },
            }
        })
        .collect::<Vec<_>>();

    Ok(EnumRustType {
        is_exact_match: result.exact_match,
        is_root_crate: item.crate_id == 0,
        path: result.item.path.clone(),
        variants,
    })
}

fn _find_struct_with_resolved_fields(
    result: &SearchResult,
    rdocs: &Crate,
) -> Result<StructRustType, anyhow::Error> {
    let rustdoc_types::ItemKind::Struct = result.item.kind else {
        bail!("not a struct type")
    };

    let (_, item) = rdocs
        .index
        .iter()
        .find(|(_, item)| item.id.eq(result.id))
        .context("locate struct in .index")?;

    let rustdoc_types::ItemEnum::Struct(struct_) = &item.inner else {
        unreachable!("_find_struct_with_resolved_fields: must be a struct type",)
    };

    let kind = match &struct_.kind {
        rustdoc_types::StructKind::Unit => StructKind::Unit,
        rustdoc_types::StructKind::Tuple(vec) => StructKind::Tuple(_resolve_fields(
            rdocs,
            &vec.iter().flatten().copied().collect::<Vec<_>>(),
        )),
        rustdoc_types::StructKind::Plain {
            fields,
            has_stripped_fields: _,
        } => StructKind::Plain {
            fields: _resolve_fields(rdocs, fields),
        },
    };

    Ok(StructRustType {
        is_exact_match: result.exact_match,
        is_root_crate: item.crate_id == 0,
        path: result.item.path.clone(),
        kind,
    })
}

fn _resolve_fields(rdocs: &Crate, fields: &[rustdoc_types::Id]) -> Vec<StructFieldOrEnumVariant> {
    fields
        .iter()
        .flat_map(|id| rdocs.index.get(id))
        .map(|item| {
            let rustdoc_types::ItemEnum::StructField(ty) = &item.inner else {
                unreachable!("_resolve_fields: must be a struct field")
            };

            let kind = match ty {
                rustdoc_types::Type::ResolvedPath(path) => {
                    StructFieldKind::ResolvedPath { path: path.clone() }
                }
                rustdoc_types::Type::Primitive(ty) => {
                    StructFieldKind::Primitive { name: ty.clone() }
                }
                _ => unimplemented!("only struct kind plain or resolved path are supported"),
            };

            StructFieldOrEnumVariant {
                name: item.name.clone(),
                kind,
            }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone)]
pub enum RustType {
    Struct(StructRustType),
    Enum(EnumRustType),
}

#[derive(Debug, Clone)]
pub struct EnumRustType {
    pub is_exact_match: bool,
    is_root_crate: bool,
    pub path: Vec<String>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub kind: EnumVariantKind,
}

impl EnumVariant {
    pub fn are_same_kind(&self, other: &EnumVariant) -> bool {
        matches!(
            (&self.kind, &other.kind),
            (
                EnumVariantKind::Struct { .. },
                EnumVariantKind::Struct { .. }
            ) | (EnumVariantKind::Plain, EnumVariantKind::Plain)
                | (EnumVariantKind::Tuple(_), EnumVariantKind::Tuple(_))
        )
    }
}

#[derive(Debug, Clone)]
pub enum EnumVariantKind {
    Plain,
    Tuple(Vec<StructFieldOrEnumVariant>),
    Struct {
        fields: Vec<StructFieldOrEnumVariant>,
    },
}

#[derive(Debug, Clone)]
pub struct StructRustType {
    pub is_exact_match: bool,
    is_root_crate: bool,
    pub path: Vec<String>,
    pub kind: StructKind,
}

impl RustType {
    pub fn name(&self) -> &str {
        match self {
            RustType::Struct(s) => s.path.last(),
            RustType::Enum(e) => e.path.last(),
        }
        .expect("name")
    }

    pub fn is_root_crate(&self) -> bool {
        match self {
            RustType::Struct(s) => s.is_root_crate,
            RustType::Enum(e) => e.is_root_crate,
        }
    }

    pub fn exact_match(&self) -> bool {
        match self {
            RustType::Struct(s) => s.is_exact_match,
            RustType::Enum(e) => e.is_exact_match,
        }
    }

    pub fn path_segments(&self) -> &Vec<String> {
        match self {
            RustType::Struct(s) => &s.path,
            RustType::Enum(e) => &e.path,
        }
    }

    pub fn path(&self) -> syn::Path {
        let is_root_crate = self.is_root_crate();
        let mut segments = self
            .path_segments()
            .clone()
            .into_iter()
            .skip(if is_root_crate { 1 } else { 0 }) // TODO(FIX): Skip the crate name
            .collect::<Vec<_>>();
        if is_root_crate {
            segments.insert(0, "crate".to_string());
        }
        let segments = segments.join("::");
        syn::parse_str(&segments).expect("parse path")
    }

    pub fn are_same_kind(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (RustType::Struct(_), RustType::Struct(_)) | (RustType::Enum(_), RustType::Enum(_))
        )
    }
}

#[derive(Debug, Clone)]
pub enum StructKind {
    Unit,
    Tuple(Vec<StructFieldOrEnumVariant>),
    Plain {
        fields: Vec<StructFieldOrEnumVariant>,
    },
}

#[derive(Debug, Clone)]
pub struct StructFieldOrEnumVariant {
    /// Unset for tuple fields
    pub name: Option<String>,
    pub kind: StructFieldKind,
}

#[derive(Debug, Clone)]
pub enum StructFieldKind {
    Primitive { name: String },
    ResolvedPath { path: rustdoc_types::Path },
}

impl StructFieldOrEnumVariant {
    pub fn t_of_option(&self) -> anyhow::Result<&rustdoc_types::Path> {
        let StructFieldKind::ResolvedPath { path: source_path } = &self.kind else {
            anyhow::bail!("must be a resolved path")
        };

        let Some(generic_args) = &source_path.args else {
            anyhow::bail!("unreachable(BUG): must have args");
        };
        let GenericArgs::AngleBracketed {
            args,
            constraints: _,
        } = generic_args.as_ref()
        else {
            dbg!(generic_args);

            anyhow::bail!("unimplemented: Option type of parentized argument")
        };
        let Some(GenericArg::Type(ty)) = args.iter().next() else {
            dbg!(generic_args);

            anyhow::bail!("unimplemented: Option type with infered, const or lifetype argument")
        };

        let rustdoc_types::Type::ResolvedPath(path) = ty else {
            dbg!(generic_args);

            anyhow::bail!("unimplemented: Option type with touple value: Option<(,)>")
        };

        Ok(path)
    }
}

impl StructFieldKind {
    pub fn as_str(&self) -> &str {
        match self {
            StructFieldKind::Primitive { name } => name,
            StructFieldKind::ResolvedPath { path } => &path.name,
        }
    }
    pub fn is_same_kind(&self, other: &StructFieldKind) -> bool {
        matches!(
            (self, other),
            (
                StructFieldKind::Primitive { .. },
                StructFieldKind::Primitive { .. }
            ) | (
                StructFieldKind::ResolvedPath { .. },
                StructFieldKind::ResolvedPath { .. }
            )
        )
    }

    pub fn are_both_option_type(item1: &StructFieldKind, item2: &StructFieldKind) -> bool {
        match (item1, item2) {
            (
                StructFieldKind::ResolvedPath { path: p1 },
                StructFieldKind::ResolvedPath { path: p2 },
            ) => p1.name.clone() == "Option" && p2.name == "Option",
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, StructFieldKind::Primitive { .. })
    }

    pub fn is_resolved_path(&self) -> bool {
        matches!(self, StructFieldKind::ResolvedPath { .. })
    }

    pub fn is_primitive_eq(&self, other: &StructFieldKind) -> bool {
        match (self, other) {
            (StructFieldKind::Primitive { name: a }, StructFieldKind::Primitive { name: b }) => {
                a == b
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use quote::format_ident;

    use super::*;

    #[test]
    fn find_struct() {
        let rdoc = get_test_data();
        let struct_ = query_structs(&format_ident!("SourceStruct").into(), &rdoc).unwrap();
        assert_eq!(struct_.len(), 1);
        assert!(!struct_[0].is_exact_match);

        let struct_ = query_structs(
            &syn::parse_str("usage::v2::models::SourceStruct").unwrap(),
            &rdoc,
        )
        .unwrap();
        assert_eq!(struct_.len(), 1);
        assert!(struct_[0].is_exact_match);
    }

    pub(crate) fn get_test_data() -> Crate {
        let json = include_str!("../../usage/rustdoc_v2.json");
        serde_json::from_str(json).unwrap()
    }
}

pub trait KindAsStr {
    fn kind_as_str(&self) -> &'static str;
}

impl KindAsStr for EnumVariant {
    fn kind_as_str(&self) -> &'static str {
        match self.kind {
            EnumVariantKind::Plain => "Plain",
            EnumVariantKind::Tuple(_) => "Tuple",
            EnumVariantKind::Struct { .. } => "Struct",
        }
    }
}

impl KindAsStr for RustType {
    fn kind_as_str(&self) -> &'static str {
        match self {
            RustType::Struct(_) => "Struct",
            RustType::Enum(_) => "Enum",
        }
    }
}

impl KindAsStr for StructKind {
    fn kind_as_str(&self) -> &'static str {
        match self {
            StructKind::Unit => "Unit",
            StructKind::Tuple(_) => "Tuple",
            StructKind::Plain { .. } => "Plain",
        }
    }
}
