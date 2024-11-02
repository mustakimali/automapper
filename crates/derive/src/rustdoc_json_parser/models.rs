use quote::format_ident;

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
    pub fn type_name(&self) -> syn::Ident {
        match &self.ty {
            StructFieldKind::Primitive(name) => format_ident!("{}", name),
            StructFieldKind::ResolvedPath(r) => format_ident!("{}", r.name),
        }
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
