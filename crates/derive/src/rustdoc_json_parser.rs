use std::collections::HashMap;

use anyhow::Context;
use models::*;
use serde_json::Value;

mod models;

pub fn find_fields_of_struct(name: &str, rustdoc: &Value) -> anyhow::Result<Vec<StructField>> {
    let all_fields = parse_all_struct_fields(rustdoc)?
        .into_iter()
        .map(|f| (f.id.clone(), f))
        .collect::<HashMap<_, _>>();

    enumerate_structs(rustdoc)?
        .filter(|s| &s.name == name)
        .map(|s| s.field_ids)
        .next()
        .map(|field_ids| {
            field_ids
                .into_iter()
                .flat_map(|f| all_fields.get(f.as_str()).cloned())
                .collect::<Vec<_>>()
        })
        .context("locate struct")
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
        .flat_map(|(_, root_item)| parse_struct_field(root_item).ok())
        .collect::<Vec<_>>();

    Ok(structs)
}

fn parse_struct(rustdoc: &Value) -> anyhow::Result<Struct> {
    let struct_plain_fields = rustdoc
        .navigate(&["inner", "struct", "kind", "plain", "fields"])?
        .as_array()
        .context("cast .inner.struct.kind.plain.fields as array")?
        .iter()
        .map(|f| f.as_str())
        .flatten()
        .map(|f| f.to_string())
        .collect::<Vec<_>>();
    Ok(Struct {
        name: rustdoc.get_str("name")?,
        field_ids: struct_plain_fields,
    })
}

fn parse_struct_field(rustdoc: &Value) -> anyhow::Result<StructField> {
    let struct_field = rustdoc
        .navigate(&["inner", "struct_field"])
        .ok()
        .and_then(|f| serde_json::from_value(f.clone()).ok())
        .context("locate .inner.struct_field")?;

    Ok(StructField {
        id: rustdoc.get_str("id")?,
        name: rustdoc.get_str("name")?,
        ty: struct_field,
    })
}

trait ValueExt {
    fn get_str(&self, key: &str) -> anyhow::Result<String>;
    fn navigate(&self, keys: &[&str]) -> anyhow::Result<&Value>;
}

impl ValueExt for &Value {
    fn get_str(&self, key: &str) -> anyhow::Result<String> {
        self.get(key)
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .context(format!("locate .{}", key))
    }

    fn navigate(&self, keys: &[&str]) -> anyhow::Result<&Value> {
        keys.iter().fold(Ok(self), |acc, key| {
            acc.and_then(|v| v.get(key).context(format!("locate .{}", key)))
        })
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_find_fields_of_struct() {
        let rustdoc = get_test_data();

        let fields = find_fields_of_struct("Test", &rustdoc).unwrap();
        assert_eq!(fields.len(), 4);
        assert_eq!(
            fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>(),
            vec!["left", "right", "expected", "name"]
        );
    }

    #[test]
    fn test_find_all_structs() {
        let rustdoc = get_test_data();

        let structs = find_all_structs(&rustdoc).unwrap();
        assert_eq!(structs.len(), 2);
        assert_eq!(
            structs.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
            vec!["Test", "Test2"]
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
}
