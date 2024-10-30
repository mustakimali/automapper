#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub field_ids: Vec<String>,
}

#[derive(Debug)]
pub struct StructField {
    pub id: String,
    pub name: String,
    pub ty: StructFieldKind,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructFieldKind {
    Primitive(String),
    ResolvedPath(ResolvedPathStructField),
}

#[derive(Debug, serde::Deserialize)]
pub struct ResolvedPathStructField {
    pub id: String,
    pub name: String,
}
