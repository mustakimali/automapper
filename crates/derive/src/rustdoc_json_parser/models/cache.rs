use super::{FqIdent, RustType};

#[derive(Debug)]
pub struct PathCache {
    path: Vec<FqIdent>,
}

#[derive(Debug)]
pub struct TypeCache {
    path: Vec<RustType>,
}

impl TypeCache {
    pub fn new(types: Vec<RustType>) -> Self {
        Self { path: types }
    }

    pub fn find(&self, ident: &FqIdent) -> Option<&RustType> {
        self.path.iter().find(|i| i.name() == ident.to_string())
    }
}

impl PathCache {
    pub fn new(idents: Vec<FqIdent>) -> Self {
        // todo: build fast lookup cache

        Self { path: idents }
    }

    pub fn find(&self, name: &[syn::Ident]) -> Option<&FqIdent> {
        self.path.iter().find(|i| i.path.ends_with(name))
    }

    pub fn find_by_path(&self, name: &syn::Path) -> Option<&FqIdent> {
        let seg = name
            .segments
            .iter()
            .map(|s| s.ident.clone())
            .collect::<Vec<_>>();

        self.find(&seg)
    }
}

#[cfg(test)]
mod test {
    use quote::format_ident;

    use super::*;

    #[test]
    fn search() {
        let cache = PathCache::new(vec![FqIdent::from_idents(vec![
            format_ident!("std"),
            format_ident!("path"),
            format_ident!("Path"),
        ])]);

        assert!(cache.find(&[format_ident!("Path")]).is_some());
        //assert!(cache.find_by_path(syn::Path::parse).is_some()) // todo: test
    }
}
