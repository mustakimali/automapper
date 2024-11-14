use anyhow::{anyhow, bail, Context};
use rustdoc_types::{Crate, ItemSummary};

fn find_struct_by_name(name: syn::Path, rdoc: &Crate) -> anyhow::Result<StructWrapper> {
    let (exact_match, path_entry) = {
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

    let rustdoc_types::ItemKind::Struct = path_entry.kind else {
        bail!("not a struct type")
    };

    let (id, item) = rdoc
        .index
        .iter()
        .find(|(_, item)| item.id == item.id)
        .context("locate struct in .index")?;

    dbg!(&item);

    todo!()
}

struct StructWrapper {
    is_exact_match: bool,
}

#[cfg(test)]
mod test {
    use quote::format_ident;

    use super::*;

    #[test]
    fn find_struct() {
        let rdoc = get_test_data();
        find_struct_by_name(format_ident!("Test").into(), &rdoc);
    }

    pub(crate) fn get_test_data() -> Crate {
        let json = include_str!("../../usage/rustdoc.json");
        serde_json::from_str(json).unwrap()
    }
}
