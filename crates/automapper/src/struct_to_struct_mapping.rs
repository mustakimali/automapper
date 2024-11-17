use anyhow::Context;
use quote::{quote, ToTokens};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, StructWrapper},
};

pub struct StructToStructMapping {
    source: StructWrapper,
    dest: StructWrapper,
    ctx: MacroCtx,
}

impl StructToStructMapping {
    pub fn new(
        source_path: syn::Path,
        dest_path: syn::Path,
        ctx: MacroCtx,
    ) -> anyhow::Result<Self> {
        let source = rodc_util::find_struct_by_exact_name(&source_path, &ctx.rdocs)
            .with_context(|| {
                format!(
                    "failed to find source struct: {}",
                    source_path.to_token_stream().to_string()
                )
            })
            .unwrap();

        let dest = rodc_util::find_struct_by_exact_name(&dest_path, &ctx.rdocs)
            .with_context(|| {
                format!(
                    "failed to find dest struct: {}",
                    dest_path.to_token_stream().to_string()
                )
            })
            .unwrap();

        Ok(Self { source, dest, ctx })
    }
}

impl ToTokens for StructToStructMapping {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let source = &self.source;
        let dest = &self.dest;
        let ctx = &self.ctx;

        tokens.extend(quote! {
            todo!()
        });
    }
}
