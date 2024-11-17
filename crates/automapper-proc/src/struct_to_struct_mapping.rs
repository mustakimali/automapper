use anyhow::Context;
use quote::{format_ident, quote, ToTokens};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, StructWrapper},
};

pub struct StructToStructMapping {
    source: StructWrapper,
    /// The path to the accessor function for the source struct.
    /// Starting from `value` in the root mapping function.
    source_accessor: Vec<String>,
    dest: StructWrapper,
    ctx: MacroCtx,
}

impl StructToStructMapping {
    pub fn new(
        source_path: syn::Path,
        source_accessor: Vec<String>,
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

        Ok(Self {
            source,
            source_accessor,
            dest,
            ctx,
        })
    }
}

impl ToTokens for StructToStructMapping {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let source = &self.source;
        let dest = &self.dest;

        let dest_path = dest.path();

        match &dest.kind {
            rodc_util::StructKind::Unit => {
                tokens.extend(quote! {
                    #dest_path // unit struct
                });
            }
            rodc_util::StructKind::Tuple(_vec) => todo!(),
            rodc_util::StructKind::Plain { fields } => {
                let rodc_util::StructKind::Plain {
                    fields: source_fields,
                } = &source.kind
                else {
                    panic!("source struct is not plain struct");
                };
                let accessor = self
                    .source_accessor
                    .iter()
                    .map(|i| format_ident!("{}", i))
                    .collect::<Vec<_>>();
                let accessor = quote! { #(#accessor).* };

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

                    if !dest_f.kind.is_same_kind(&matching_source_f.kind) {
                        panic!(
                            "source and dest fields are not of the same kind: {} and {}",
                            dest_f.kind.as_str(),
                            matching_source_f.kind.as_str()
                        );
                    }

                    match &dest_f.kind {
                        rodc_util::StructFieldKind::Primitive { name: _ } => {
                            if dest_f.kind.is_primitive_eq(&matching_source_f.kind) {
                                // primitive types: can be directly assigned

                                mappings.push(quote! {
                                    #dest_f_name: #accessor.#source_f_name, /* primative type */
                                });
                            } else {
                                // primitive types: may require explicit casting
                                //TODO(FIX): only castable types
                                mappings.push(quote! {
                                    #dest_f_name: #accessor.#source_f_name as _, /* primative type with casting */
                                });
                            }
                        }
                        rodc_util::StructFieldKind::ResolvedPath { path: dest_path } => {
                            let rodc_util::StructFieldKind::ResolvedPath { path: source_path } =
                                &matching_source_f.kind
                            else {
                                unreachable!("must be resolved path")
                            };

                            if dest_path.id == source_path.id {
                                // same path: can be directly assigned
                                mappings.push(quote! {
                                    #dest_f_name: #accessor.#source_f_name, /* primative type */
                                });
                                continue;
                            }

                            // resolve_path
                            let new_source_accessor = {
                                let mut s = self.source_accessor.clone();
                                s.push(source_f_name.to_string());
                                s
                            };
                            let struct_mapping = StructToStructMapping::new(
                                rodc_util::find_path_by_id(&source_path.id, &self.ctx.rdocs),
                                new_source_accessor,
                                rodc_util::find_path_by_id(&dest_path.id, &self.ctx.rdocs),
                                self.ctx.clone(),
                            )
                            .with_context(|| {
                                format!(
                                    "failed to create mapping for source: {} and dest: {}",
                                    matching_source_f.name.clone().unwrap_or_default(),
                                    dest_f.name.clone().unwrap_or_default()
                                )
                            })
                            .unwrap();

                            mappings.push(quote! {
                                #dest_f_name: #struct_mapping,
                            });
                        }
                    }
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
