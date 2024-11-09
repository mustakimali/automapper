use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use derivative::Derivative;
use serde_json::Value;

use super::cache::Cache;
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
    pub fn count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MacroContextInner {
    #[derivative(Debug = "ignore")]
    pub cache: Cache,
}

impl Deref for MacroCtx {
    type Target = MacroContextInner;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}
