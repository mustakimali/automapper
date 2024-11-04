use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FqIdent {
    pub path: Vec<syn::Ident>,
}

impl FqIdent {
    pub fn try_from(name: &str) -> anyhow::Result<Self> {
        let path = name
            .split("::")
            .map(|s| format_ident!("{}", s))
            .collect::<Vec<_>>();
        if path.is_empty() {
            anyhow::bail!("invalid identifier empty path");
        }
        Ok(Self { path })
    }

    pub fn from_idents(idents: Vec<syn::Ident>) -> Self {
        Self { path: idents }
    }

    pub fn from_path(path: syn::Path) -> Self {
        let segments = path
            .segments
            .iter()
            .map(|s| s.ident.clone())
            .collect::<Vec<_>>();
        Self { path: segments }
    }

    pub fn name(&self) -> &[syn::Ident] {
        &self.path
    }

    pub fn name_string(&self) -> String {
        self.path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    pub fn maybe_eq(&self, other: &Self) -> bool {
        self.path == other.path || self.path.last() == other.path.last()
    }

    pub fn crate_scoped(&self) -> Self {
        let mut path = self.path.clone();
        path.remove(0);
        path.insert(0, format_ident!("crate"));
        Self { path }
    }
}

impl ToTokens for FqIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;
        tokens.extend(quote! {#(#path)::*});
    }
}

impl std::fmt::Display for FqIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, ident) in self.path.iter().enumerate() {
            if i > 0 {
                write!(f, "::")?;
            }
            write!(f, "{}", ident)?;
        }
        Ok(())
    }
}
