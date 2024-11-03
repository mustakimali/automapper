use super::FqIdent;

#[derive(Debug)]
pub struct PathCache {
    path: Vec<FqIdent>,
}

impl PathCache {
    pub fn new(idents: Vec<FqIdent>) -> Self {
        for iden in &idents {}

        Self { path: idents }
    }
}
