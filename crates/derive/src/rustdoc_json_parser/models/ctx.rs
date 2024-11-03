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
struct MacroContextInner {
    pub rustdoc: Value,
    pub path_cache: PathCache,
}
