use anyhow::Context;
use rustdoc_types::{Crate, ItemSummary};

fn find_struct_by_name(name: syn::Path, rdoc: &Crate) -> anyhow::Result<StructWrapper> {
    let (exact_match, item) = {
        let segments = name
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>();

        rdoc.paths
            .iter()
            .find(|(_, p)| p.path == segments)
            .map(|(_, m)| (true, m))
            .or_else(|| {
                rdoc.paths
                    .iter()
                    .find(|(_, p)| p.path.ends_with(&segments))
                    .map(|(_, m)| (false, m))
            })
    }
    .context("find struct by name")?;

    match item.kind {
        rustdoc_types::ItemKind::Struct => todo!(),

        _ => unimplemented!(),
    }

    todo!()
}

struct StructWrapper {
    is_exact_match: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    pub(crate) fn get_test_data() -> Crate {
        let json = include_str!("../../usage/rustdoc.json");
        serde_json::from_str(json).unwrap()
    }
}
