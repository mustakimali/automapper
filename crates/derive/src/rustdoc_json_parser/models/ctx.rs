use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use serde_json::Value;

use super::PathCache;

#[derive(Clone, Debug)]
pub struct MacroCtx(Rc<MacroContextInner>);

impl MacroCtx {
    pub fn new(ctx: MacroContextInner) -> Self {
        Self(Rc::new(ctx))
    }
    pub fn inner(&self) -> &MacroContextInner {
        &self.0
    }
}

#[derive(Debug)]
pub struct MacroContextInner {
    pub rustdoc_json: Value,
    pub path_cache: PathCache,
}

impl Deref for MacroCtx {
    type Target = MacroContextInner;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}