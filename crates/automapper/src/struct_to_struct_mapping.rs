use anyhow::Context;
use quote::{format_ident, quote, ToTokens};

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

        let dest_path = dest.path();

        match &dest.kind {
            rodc_util::StructKind::Unit => {
                tokens.extend(quote! {
                    #dest_path
                });
            }
            rodc_util::StructKind::Tuple(vec) => todo!(),
            rodc_util::StructKind::Plain { fields } => {
                let rodc_util::StructKind::Plain {
                    fields: source_fields,
                } = &source.kind
                else {
                    panic!("source struct is not plain struct");
                };

                let mut mappings = Vec::with_capacity(fields.len());
                for dest_f in fields.iter() {
                    let Some(matching_source_f) =
                        source_fields.iter().find(|f| f.name == dest_f.name)
                    else {
                        panic!(
                            "failed to find matching source field for dest field: {}",
                            dest_f.name.clone().unwrap_or_default() // must be Some(_) for Plain struct fields
                        );
                    };

                    let dest_f_name = format_ident!("{}", dest_f.name.clone().unwrap_or_default());
                    let source_f_name =
                        format_ident!("{}", matching_source_f.name.clone().unwrap_or_default());

                    mappings.push(quote! {
                        #dest_f_name: value.#source_f_name,
                    });
                }

                tokens.extend(quote! {
                    #dest_path {
                        #(#mappings)*
                    }
                });
            }
        }
    }
}
