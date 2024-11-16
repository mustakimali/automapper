use anyhow::{anyhow, bail, Context};
use rustdoc_types::{Crate, Item, ItemSummary};

fn find_struct_by_name(name: syn::Path, rdocs: &Crate) -> anyhow::Result<StructWrapper> {
    let (exact_match, path_entry_id, path_entry) = {
        let segments = name
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>();

        rdocs
            .paths
            .iter()
            .find(|(_, p)| p.path == segments)
            .map(|(id, m)| (true, id, m))
            .or_else(|| {
                rdocs
                    .paths
                    .iter()
                    .find(|(_, p)| p.path.ends_with(&segments))
                    .map(|(id, m)| (false, id, m))
            })
    }
    .context("find struct by name")?;

    dbg!(&exact_match, &path_entry);

    let rustdoc_types::ItemKind::Struct = path_entry.kind else {
        bail!("not a struct type")
    };

    let (_, item) = rdocs
        .index
        .iter()
        .find(|(_, item)| item.id.eq(path_entry_id))
        .context("locate struct in .index")?;

    let rustdoc_types::ItemEnum::Struct(struct_) = &item.inner else {
        unreachable!("must be a struct type",)
    };

    fn resolve_fields(rdocs: &Crate, fields: &[rustdoc_types::Id]) -> Vec<StructField> {
        fields
            .iter()
            .flat_map(|id| rdocs.index.get(id))
            .map(|item| {
                let rustdoc_types::ItemEnum::StructField(ty) = &item.inner else {
                    unreachable!("must be a struct field")
                };
                dbg!(&ty);
                let kind = match ty {
                    rustdoc_types::Type::ResolvedPath(path) => {
                        StructFieldKind::ResolvedPath { path: path.clone() }
                    }
                    rustdoc_types::Type::Primitive(ty) => {
                        StructFieldKind::Primitive { name: ty.clone() }
                    }
                    _ => unimplemented!("only struct field with resolved path is supported"),
                };

                StructField {
                    name: item.name.clone(),
                    kind,
                }
            })
            .collect::<Vec<_>>()
    }

    let kind = match &struct_.kind {
        rustdoc_types::StructKind::Unit => StructKind::Unit,
        rustdoc_types::StructKind::Tuple(vec) => StructKind::Tuple(resolve_fields(
            rdocs,
            &vec.iter().flatten().map(|c| c.clone()).collect::<Vec<_>>(),
        )),
        rustdoc_types::StructKind::Plain {
            fields,
            has_stripped_fields: _,
        } => StructKind::Plain {
            fields: resolve_fields(rdocs, &fields),
        },
    };

    Ok(StructWrapper {
        is_exact_match: exact_match,
        path: path_entry.path.clone(),
        kind,
    })
}

#[derive(Debug)]
struct StructWrapper {
    is_exact_match: bool,
    path: Vec<String>,
    kind: StructKind,
}

#[derive(Debug)]
enum StructKind {
    Unit,
    Tuple(Vec<StructField>),
    Plain { fields: Vec<StructField> },
}

#[derive(Debug)]
struct StructField {
    /// Unset for tuple fields
    name: Option<String>,
    kind: StructFieldKind,
}

#[derive(Debug)]
enum StructFieldKind {
    Primitive { name: String },
    ResolvedPath { path: rustdoc_types::Path },
}

#[cfg(test)]
mod test {
    use quote::format_ident;

    use super::*;

    #[test]
    fn find_struct() {
        let rdoc = get_test_data();
        let struct_ = find_struct_by_name(format_ident!("Test").into(), &rdoc);
        dbg!(struct_);
        panic!();
    }

    pub(crate) fn get_test_data() -> Crate {
        let json = include_str!("../../usage/rustdoc.json");
        serde_json::from_str(json).unwrap()
    }
}
