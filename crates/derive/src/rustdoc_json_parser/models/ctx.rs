use std::sync::Arc;

use serde_json::Value;

use super::PathCache;

#[derive(Clone, Debug)]
pub struct MacroCtx(Arc<MacroContextInner>);

impl MacroCtx {
    pub fn inner(&self) -> &MacroContextInner {
        &self.0
    }
}

#[derive(Debug)]
pub struct MacroContextInner {
    rustdoc: Value,
    path_cache: PathCache,
}
