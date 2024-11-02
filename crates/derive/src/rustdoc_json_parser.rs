use std::collections::HashMap;

use anyhow::Context;
use models::*;
use quote::format_ident;
use serde_json::Value;
use thiserror::__private::AsDisplay;

pub mod models;

pub fn find_struct_and_resolve_fields_for_ident(
    fq_ident: &FqIdent,
    rustdoc: &Value,
) -> anyhow::Result<(Struct, Vec<StructField>)> {
    find_struct_and_resolve_fields(fq_ident, rustdoc)
}

pub fn find_struct_and_resolve_fields(
    fq_ident: &FqIdent,
    rustdoc: &Value,
) -> anyhow::Result<(Struct, Vec<StructField>)> {
    let all_fields = parse_all_struct_fields(rustdoc)?
        .into_iter()
        .map(|f| (f.id.clone(), f))
        .collect::<HashMap<_, _>>();

    let mut found_struct: Option<Struct> = None;

    let fields = enumerate_structs(rustdoc)?
        .filter(|s| s.name_ident().maybe_eq(fq_ident))
        .map(|s| {
            found_struct = Some(s.clone());
            s
        })
        .map(|s| s.field_ids)
        .next()
        .map(|field_ids| {
            field_ids
                .into_iter()
                .flat_map(|f| all_fields.get(f.as_str()).cloned())
                .collect::<Vec<_>>()
        })
        .with_context(|| {
            format!(
                "error finding fields for struct `{}`",
                fq_ident.name_string()
            )
        })?;

    found_struct.map(|s| (s, fields)).context("locate struct")
}

pub fn find_all_structs(rustdoc: &Value) -> anyhow::Result<Vec<Struct>> {
    let structs = enumerate_structs(rustdoc)?;

    Ok(structs.collect::<Vec<_>>())
}

fn enumerate_structs(
    rustdoc: &Value,
) -> anyhow::Result<impl std::iter::Iterator<Item = Struct> + use<'_>> {
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;
    let structs = index
        .iter()
        .flat_map(|(_, root_item)| parse_struct(root_item).ok());
    Ok(structs)
}

pub fn parse_all_struct_fields(rustdoc: &Value) -> anyhow::Result<Vec<StructField>> {
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;

    let structs = index
        .iter()
        .flat_map(|(_id, root_item)| parse_struct_field(root_item, rustdoc).ok())
        .collect::<Vec<_>>();

    Ok(structs)
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
        .collect::<Vec<_>>();
    Ok(Struct {
        name: rustdoc.get_str("name")?,
        field_ids: struct_plain_fields,
    })
}

fn parse_struct_field(rustdoc_node: &Value, rustdoc_root: &Value) -> anyhow::Result<StructField> {
    let struct_field = rustdoc_node
        .navigate(&["inner", "struct_field"])
        .ok()
        .and_then(|f| serde_json::from_value(f.clone()).ok())
        .context("locate .inner.struct_field")?;
    let extn_crate_name = match &struct_field {
        StructFieldKind::Primitive(_) => None,
        StructFieldKind::ResolvedPath(r) => rustdoc_root
            .navigate(&["external_crates", &r.id.to_string()])
            .ok()
            .and_then(|n| n.get_str("name").ok()),
    };

    Ok(StructField {
        id: rustdoc_node.get_str("id")?,
        name: rustdoc_node.get_str("name")?,
        ty: struct_field,
        external_crate_name: extn_crate_name,
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
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_find_all_structs() {
        let rustdoc = get_test_data();

        let structs = find_all_structs(&rustdoc).unwrap();
        assert_eq!(structs.len(), 2);
        assert_eq!(
            structs.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
            vec!["Test2", "Test"]
        );
    }

    #[test]
    fn test_find_struct_and_resolve_field() {
        let rustdoc = get_test_data();

        verify_fields_of_struct(&rustdoc, "Test", &["left", "right", "expected", "name"]);

        let rustdoc = get_real_data();

        verify_fields_of_struct(&rustdoc, "Test", &["left", "right", "expected", "name"]);
        verify_fields_of_struct(&rustdoc, "Test2", &["left", "right", "expected", "name"]);
    }

    #[test]
    fn test_find_struct_with_nested_field_and_resolve_field() {
        let rustdoc = get_real_data();

        let (structs, fields) =
            find_struct_and_resolve_fields("TestNestedField", &rustdoc).unwrap();
        assert_eq!(5, fields.len());
        let mut fields = fields.into_iter();
        fields.next().unwrap(); // left
        fields.next().unwrap(); // right
        fields.next().unwrap(); // expected

        let nested_test = fields.next().unwrap();
        dbg!(&nested_test);
        assert_eq!(
            nested_test.external_crate_name.unwrap(),
            "alloc".to_string()
        );
        match &nested_test.ty {
            StructFieldKind::ResolvedPath(ResolvedPathStructField { name, .. }) => {
                assert_eq!(name, &"Test".to_string());
            }
            _ => panic!("Expected ResolvedPath"),
        }

        let nested_extn_crate = fields.next().unwrap();
        dbg!(&nested_extn_crate);
        assert!(nested_extn_crate.external_crate_name.is_none());

        match &nested_extn_crate.ty {
            StructFieldKind::ResolvedPath(ResolvedPathStructField { name, .. }) => {
                assert_eq!(name, &"std::path::PathBuf".to_string());
            }
            _ => panic!("Expected ResolvedPath"),
        }
    }

    fn verify_fields_of_struct(rustdoc: &Value, struct_name: &str, expected_fields: &[&str]) {
        let (structs, fields) = find_struct_and_resolve_fields("Test", &rustdoc).unwrap();
        assert_eq!(structs.name, "Test".to_string());
        assert_eq!(
            fields.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
            expected_fields
        );
    }

    #[test]
    fn test_parse_all_struct_fields() {
        let rustdoc = get_test_data();

        let fields = parse_all_struct_fields(&rustdoc).unwrap();
        assert_eq!(fields.len(), 8);
    }

    fn get_test_data() -> Value {
        let json = include_str!("../rustdoc_sample.json");
        let rustdoc: Value = serde_json::from_str(json).unwrap();
        rustdoc
    }

    fn get_real_data() -> Value {
        let json = include_str!("../../usage/rustdoc.json");
        let rustdoc: Value = serde_json::from_str(json).unwrap();
        rustdoc
    }
}
