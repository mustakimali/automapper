use anyhow::anyhow;
use rustdoc_types::{Crate, ItemSummary};

pub struct SearchResult<'a> {
    pub exact_match: bool,
    pub id: &'a rustdoc_types::Id,
    pub item: &'a ItemSummary,
}

pub fn query_items<'c>(
    name: &syn::Path,
    rdocs: &'c Crate,
) -> anyhow::Result<Vec<SearchResult<'c>>> {
    let mut segments = name
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>();

    if segments.first().ok_or(anyhow!("empty name"))?.as_str() == "crate" {
        segments.remove(0);
    }

    let result = rdocs
        .paths
        .iter()
        .find(|(_, p)| p.path == segments)
        .map(|(id, m)| vec![(true, id, m)])
        .or_else(|| {
            Some(
                rdocs
                    .paths
                    .iter()
                    .filter(|(_, p)| p.path.ends_with(&segments))
                    .map(|(id, m)| (false, id, m))
                    .collect::<Vec<_>>(),
            )
        })
        .map(|x| {
            x.into_iter()
                .map(|(exact_match, id, item)| SearchResult {
                    exact_match,
                    id,
                    item,
                })
                .collect::<Vec<_>>()
        });
    Ok(result.unwrap_or_default())
}
