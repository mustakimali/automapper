use std::{ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct MacroCtx {
    inner: Rc<MacroCtxInner>,
}

impl MacroCtx {
    pub fn new(rdocs: rustdoc_types::Crate) -> Self {
        Self {
            inner: Rc::new(MacroCtxInner { rdocs }),
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
}
