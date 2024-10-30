use anyhow::Context;
use serde_json::Value;

pub fn find_fields_of_struct(name: &str, rustdoc: &Value) -> anyhow::Result<Vec<String>> {
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;

    for (key, root_item) in index {
        let Ok(struct_) = parse_struct(root_item) else {
            continue;
        };

        if struct_.name == name {
            return Ok(struct_.field_ids);
        }
    }

    return Ok(vec![]);
}

pub fn find_all_structs(rustdoc: &Value) -> anyhow::Result<Vec<Struct>> {
    let index = rustdoc
        .get("index")
        .context("locate .index")?
        .as_object()
        .context("parse .index as object")?;

    let structs = index
        .iter()
        .flat_map(|(_, root_item)| parse_struct(root_item).ok())
        .collect::<Vec<_>>();

    Ok(structs)
}

#[derive(Debug)]
struct Struct {
    name: String,
    field_ids: Vec<String>,
}

#[derive(Debug)]
struct StructField {
    id: String,
    name: String,
    ty: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum StructFieldKind {
    Primitive(String),
    ResolvedPath(ResolvedPathStructField),
}

#[derive(Debug, serde::Deserialize)]
struct ResolvedPathStructField {
    id: String,
    name: String,
}

fn parse_struct(rustdoc: &Value) -> anyhow::Result<Struct> {
    let struct_plain_fields = rustdoc
        .get("inner")
        .and_then(|i| i.get("struct"))
        .and_then(|s| s.get("kind"))
        .and_then(|s| s.get("plain"))
        .and_then(|s| s.get("fields"))
        .and_then(|f| f.as_array())
        .context("locate .inner.struct.kind.plain.fields")?
        .iter()
        .map(|f| f.as_str())
        .flatten()
        .map(|f| f.to_string())
        .collect::<Vec<_>>();
    Ok(Struct {
        name: rustdoc.get("name").context("locate .name")?.to_string(),
        field_ids: struct_plain_fields,
    })
}

fn parse_struct_field(rustdoc: &Value) -> anyhow::Result<StructField> {
    let struct_plain_fields = rustdoc
        .get("inner")
        .and_then(|i| i.get("struct_field"))
        .and_then(|s| s.get("kind"))
        .and_then(|s| s.get("plain"))
        .and_then(|s| s.get("fields"))
        .and_then(|f| f.as_array())
        .context("locate .inner.struct.kind.plain.fields")?
        .iter()
        .map(|f| f.as_str())
        .flatten()
        .map(|f| f.to_string())
        .collect::<Vec<_>>();
    Ok(Struct {
        name: rustdoc.get("name").context("locate .name")?.to_string(),
        field_ids: struct_plain_fields,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_fields_of_struct() {
        let rustdoc = get_test_data();

        let fields = find_fields_of_struct("Test", &rustdoc).unwrap();
        assert_eq!(fields, vec!["left", "right", "expected", "name"]);
    }

    #[test]
    fn test_find_all_structs() {
        let rustdoc = get_test_data();

        let structs = find_all_structs(&rustdoc).unwrap();
        panic!("{:?}", structs);
    }

    fn get_test_data() -> Value {
        let json = include_str!("../rustdoc_sample.json");
        let rustdoc: Value = serde_json::from_str(json).unwrap();
        rustdoc
    }
}
