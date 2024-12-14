use std::{ops::Deref, rc::Rc};

use crate::Mapping;

#[derive(Clone)]
pub struct MacroCtx {
    inner: Rc<MacroCtxInner>,
}

impl MacroCtx {
    pub fn new(rdocs: rustdoc_types::Crate, custom_mappings: Option<Vec<Mapping>>) -> Self {
        Self {
            inner: Rc::new(MacroCtxInner {
                rdocs,
                custom_mappings,
            }),
        }
    }
}

impl Deref for MacroCtx {
    type Target = MacroCtxInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct MacroCtxInner {
    pub rdocs: rustdoc_types::Crate,
    pub custom_mappings: Option<Vec<Mapping>>,
}
