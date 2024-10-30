use anyhow::Context;
use models::*;
use serde_json::Value;

mod models;

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
    let struct_field = rustdoc
        .get("inner")
        .and_then(|i| i.get("struct_field"))
        .and_then(|f| serde_json::from_value(f.clone()).ok())
        .context("locate .inner.struct.kind.plain.fields")?;

    Ok(StructField {
        id: rustdoc
            .get("id")
            .and_then(|t| t.as_str())
            .map(|t| t.to_string())
            .context("locate .id")?,
        name: rustdoc
            .get("name")
            .and_then(|t| t.as_str())
            .map(|t| t.to_string())
            .context("locate .name")?
            .to_string(),
        ty: struct_field,
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

    #[test]
    fn test_parse_all_struct_fields() {
        let rustdoc = get_test_data();

        let fields = parse_all_struct_fields(&rustdoc).unwrap();
        panic!("{:#?}", fields);
    }

    fn get_test_data() -> Value {
        let json = include_str!("../rustdoc_sample.json");
        let rustdoc: Value = serde_json::from_str(json).unwrap();
        rustdoc
    }
}
