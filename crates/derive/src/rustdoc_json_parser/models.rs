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
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StructFieldKind {
    Primitive(String),
    ResolvedPath(ResolvedPathStructField),
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ResolvedPathStructField {
    pub id: u32,
    pub name: String,
}
