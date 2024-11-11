use std::collections::{HashMap, HashSet};

use anyhow::Context;
use models::*;
use quote::format_ident;
use serde_json::Value;
use thiserror::__private::AsDisplay;

pub mod models;

pub fn find_all_rusttype_fq_path(rustdoc: &Value) -> anyhow::Result<HashSet<FqIdent>> {
    let mut fq_idents = HashSet::new();
    let paths = rustdoc
        .get("paths")
        .context("locate .paths")?
        .as_object()
        .context("parse .paths as object")?;
    for (_, root_item) in paths.iter() {
        let Some(kind) = root_item.get_str("kind").ok() else {
            continue;
        };
        if kind == "struct" || kind == "enum" {
            let fq_path = root_item
                .get("path")
                .context("locate .path")?
                .as_array()
                .context("parse .path as array")?;
            let s = fq_path
                .iter()
                .flat_map(|s| s.as_str())
                .map(|s| format_ident!("{}", s))
                .collect::<Vec<_>>();

            fq_idents.insert(FqIdent::from_idents(s));
        }
    }

    Ok(fq_idents)
}

pub fn enumerate_rust_types<'a>(
    rustdoc: &'a Value,
    path_cache: &'a PathCache,
) -> anyhow::Result<impl std::iter::Iterator<Item = RustType> + 'a> {
    let all_fields = parse_all_struct_fields(rustdoc)?
        .into_iter()
        .map(|f| (f.id.clone(), f))
        .collect::<HashMap<_, _>>();
    let all_variants = parse_all_eunm_variants(rustdoc)?
        .into_iter()
        .map(|f| (f.id.clone(), f))
        .collect::<HashMap<_, _>>();
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;
    let rust_types = index.iter().flat_map(move |(_, root_item)| {
        parse_struct(root_item)
            .ok()
            .map(|struct_| RustType::Struct {
                fields: struct_
                    .field_ids
                    .iter()
                    .flat_map(|id| all_fields.get(&id).cloned())
                    .collect(),
                fq_path: FqIdent::try_from_str(&struct_.name)
                    .ok()
                    .and_then(|p| path_cache.find_fully_qualified_path(&p)),
                item: struct_,
            })
            .or_else(|| {
                parse_enum(root_item).ok().map(|enum_| RustType::Enum {
                    variants: enum_
                        .variant_ids
                        .iter()
                        .flat_map(|id| all_variants.get(id).cloned())
                        .collect(),
                    fq_path: FqIdent::try_from_str(&enum_.name)
                        .ok()
                        .and_then(|p| path_cache.find_fully_qualified_path(&p)),
                    item: enum_,
                })
            })
    });
    Ok(rust_types)
}

pub fn parse_all_struct_fields(rustdoc: &Value) -> anyhow::Result<Vec<StructField>> {
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;

    let structs = index
        .iter()
        .flat_map(|(_id, node)| parse_struct_field(node, rustdoc).ok())
        .collect::<Vec<_>>();

    Ok(structs)
}

pub fn parse_all_eunm_variants(rustdoc: &Value) -> anyhow::Result<Vec<EnumVariant>> {
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;

    let variants = index
        .iter()
        .flat_map(|(_id, node)| parse_enum_variant(node).ok())
        .collect::<Vec<_>>();

    Ok(variants)
}

fn parse_struct(rustdoc: &Value) -> anyhow::Result<Struct> {
    let struct_plain_fields = rustdoc
        .navigate(&["inner", "struct", "kind", "plain", "fields"])?
        .as_array()
        .context("cast .inner.struct.kind.plain.fields as array")?
        .iter()
        .map(|f| {
            f.as_str()
                .map(|s| s.to_string())
                .or_else(|| f.as_u64().map(|u| u.to_string()))
        })
        .flatten()
        .map(Id::from)
        .collect::<Vec<_>>();
    Ok(Struct {
        name: rustdoc.get_str("name")?,
        field_ids: struct_plain_fields,
    })
}

fn parse_enum(rustdoc: &Value) -> anyhow::Result<Enum> {
    let enum_variant_fields = rustdoc
        .navigate(&["inner", "enum", "variants"])?
        .as_array()
        .context("cast .inner.struct.kind.plain.fields as array")?
        .iter()
        .map(|f| {
            f.as_str()
                .map(|s| s.to_string())
                .or_else(|| f.as_u64().map(|u| u.to_string()))
        })
        .flatten()
        .map(Id::from)
        .collect::<Vec<_>>();
    Ok(Enum {
        name: rustdoc.get_str("name")?,
        variant_ids: enum_variant_fields,
    })
}

fn parse_struct_field(rustdoc_node: &Value, rustdoc_root: &Value) -> anyhow::Result<StructField> {
    let struct_field = rustdoc_node
        .navigate(&["inner", "struct_field"])
        .ok()
        .and_then(|f| serde_json::from_value(f.clone()).ok())
        .context("locate .inner.struct_field")?;
    let external_crate_name = match &struct_field {
        StructFieldKind::Primitive(_) => None,
        StructFieldKind::ResolvedPath(r) => rustdoc_root
            .navigate(&["external_crates", &r.id.to_string()])
            .ok()
            .and_then(|n| n.get_str("name").ok()),
    };

    Ok(StructField {
        id: rustdoc_node.get_str("id")?.into(),
        name: rustdoc_node.get_str("name")?,
        ty: struct_field,
        external_crate_name,
    })
}

fn parse_enum_variant(rustdoc_node: &Value) -> anyhow::Result<EnumVariant> {
    let enum_variant_type = rustdoc_node
        .navigate(&["inner", "variant"])
        .ok()
        .and_then(|f| serde_json::from_value(f.clone()).ok())
        .context("locate .inner.variant")?;

    Ok(EnumVariant {
        id: rustdoc_node.get_str("id")?.into(),
        name: rustdoc_node.get_str("name")?,
        ty: enum_variant_type,
    })
}

trait ValueExt {
    fn get_str(&self, key: &str) -> anyhow::Result<String>;
    fn navigate(&self, keys: &[&str]) -> anyhow::Result<&Value>;
}

impl ValueExt for &Value {
    fn get_str(&self, key: &str) -> anyhow::Result<String> {
        self.get(key)
            .and_then(|v| {
                v.as_str()
                    .map(|v| v.to_string())
                    .or_else(|| v.as_number().map(|n| n.to_string()))
            })
            .context(format!("locate .{}", key))
    }

    fn navigate(&self, keys: &[&str]) -> anyhow::Result<&Value> {
        keys.iter().try_fold(*self, |acc, key| {
            acc.get(key).context(format!("locate .{}", key))
        })
    }
}

#[cfg(test)]
pub mod test {
    use core::panic;

    use super::*;

    //TODO: write basic tests
    #[test]
    fn test_parse_rust_type() {
        let rustdoc = get_test_data();
        let cache = Cache::new_from_rust_doc_json(rustdoc).unwrap();

        const COUNT: usize = 21;
        assert_eq!(cache.types.len(), COUNT);

        let fq_names = cache
            .types
            .iter()
            .flat_map(|t| t.fully_qualified_path())
            .map(|t| t.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            fq_names.len(),
            COUNT,
            "All types must have fully qualified paths"
        );

        assert_eq!(
            fq_names,
            &[
                "crate::nested::nested_inner::NestedDestInnerType",
                "crate::nested::NestedSourceType",
                "crate::Test",
                "crate::Test2",
                "crate::TestNestedField",
                "crate::TestNestedFieldTo",
                "crate::DestInnerType",
                "crate::DestType",
                "crate::SourceInnerType",
                "crate::SourceType",
                "crate::SourceInnerTypeWthDifferentInnerTypeCanBeCasted",
                "crate::SourceEnum",
                "crate::SourceStructWithEnumField",
                "crate::DestEnum",
                "crate::DestStructWithEnumField",
                "crate::ComplexEnumSource",
                "crate::CmplexEnumSourceStruct",
                "crate::ComplexEnumDest",
                "crate::CmplexEnumDestStruct",
                "crate::nested::nested_inner::NestedSourceInnerType",
                "crate::nested::NestedDestType"
            ]
        );
    }

    #[test]
    fn test_type_cache() {
        let rustdoc = get_test_data();
        let cache = Cache::new_from_rust_doc_json(rustdoc).unwrap();
        let rust_type = cache.find(&FqIdent::try_from_str("Test").unwrap()).unwrap();
        assert_eq!(rust_type.name(), "Test".to_string());
    }

    pub(crate) fn get_test_data() -> Value {
        let json = include_str!("../../usage/rustdoc.json");
        let rustdoc: Value = serde_json::from_str(json).unwrap();
        rustdoc
    }
}
