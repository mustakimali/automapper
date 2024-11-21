use std::ops::ControlFlow;

use anyhow::Context;
use quote::{format_ident, quote, ToTokens};
use rustdoc_types::{GenericArg, GenericArgs};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, StructFieldKind, StructRustType},
};

pub struct StructToStructMapping {
    pub source: StructRustType,
    /// The path to the accessor function for the source struct.
    /// Starting from `value` in the root mapping function.
    source_accessor: Vec<String>,
    pub dest: StructRustType,
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
                    source_path.to_token_stream()
                )
            })
            .unwrap();

        let dest = rodc_util::find_struct_by_exact_name(&dest_path, &ctx.rdocs)
            .with_context(|| {
                format!(
                    "failed to find dest struct: {}",
                    dest_path.to_token_stream()
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

    fn map_struct_plain(
        &self,
        source: &StructRustType,
        dest_fields: &[rodc_util::StructFieldOrEnumVariant],
        tokens: &mut proc_macro2::TokenStream,
        dest_path: syn::Path,
    ) {
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

        let mut mappings = Vec::with_capacity(dest_fields.len());
        for dest_f in dest_fields.iter() {
            let Some(map) = self.create_field_mapping(source_fields, dest_f, &accessor) else {
                continue;
            };
            mappings.push(map);
        }

        tokens.extend(quote! {
            #dest_path {
                #(#mappings)*
            }
        });
    }

    fn create_field_mapping(
        &self,
        source_fields: &[rodc_util::StructFieldOrEnumVariant],
        dest_field: &rodc_util::StructFieldOrEnumVariant,
        accessor: &proc_macro2::TokenStream,
    ) -> Option<proc_macro2::TokenStream> {
        let Some(source_field) = source_fields.iter().find(|f| f.name == dest_field.name) else {
            panic!(
                "failed to find matching source field for dest field: {}",
                dest_field.name.clone().unwrap_or_default() // must be Some(_) for Plain struct fields
            );
        };
        let dest_f_name = format_ident!("{}", dest_field.name.clone().unwrap_or_default());
        let source_f_name = format_ident!("{}", source_field.name.clone().unwrap_or_default());
        if !dest_field.kind.is_same_kind(&source_field.kind) {
            panic!(
                "source and dest fields are not of the same kind: {} and {}",
                dest_field.kind.as_str(),
                source_field.kind.as_str()
            );
        }

        match &dest_field.kind {
            rodc_util::StructFieldKind::Primitive { name: _ } => {
                if dest_field.kind.is_primitive_eq(&source_field.kind) {
                    // primitive types: can be directly assigned

                    Some(quote! {
                        #dest_f_name: #accessor.#source_f_name, /* primative type */
                    })
                } else {
                    // primitive types: may require explicit casting
                    //TODO(FIX): only castable types
                    Some(quote! {
                        #dest_f_name: #accessor.#source_f_name as _, /* primative type with casting */
                    })
                }
            }
            rodc_util::StructFieldKind::ResolvedPath { path: dest_path } => {
                let rodc_util::StructFieldKind::ResolvedPath { path: source_path } =
                    &source_field.kind
                else {
                    unreachable!("must be resolved path")
                };

                // Possiblity: Option<T> or Result<T, E>
                //
                //
                if StructFieldKind::are_both_option_type(&source_field.kind, &dest_field.kind) {
                    let source_t_of_option = source_field.t_of_option().unwrap();
                    let dest_t_of_option = dest_field.t_of_option().unwrap();

                    let struct_mapping_inside_lambda = StructToStructMapping::new(
                        rodc_util::find_path_by_id(&source_t_of_option.id, &self.ctx.rdocs),
                        vec!["v".to_string()],
                        rodc_util::find_path_by_id(&dest_t_of_option.id, &self.ctx.rdocs),
                        self.ctx.clone(),
                    )
                    .with_context(|| {
                        format!(
                            "failed to create mapping for source: {} and dest: {}",
                            source_field.name.clone().unwrap_or_default(),
                            dest_field.name.clone().unwrap_or_default()
                        )
                    })
                    .unwrap();

                    return Some(quote! {
                        #dest_f_name: #accessor.#source_f_name.map(|v| {
                            #struct_mapping_inside_lambda
                        }),
                    });
                }

                // Possiblity: Same type with different generic arg
                //
                //
                // TODO: error

                // Possiblity: Same type of struct to struct mapping (non generic)
                //
                if dest_path.id == source_path.id {
                    // same path: can be directly assigned
                    return Some(quote! {
                        #dest_f_name: #accessor.#source_f_name, /* primative type */
                    });
                }

                // Possiblity: Other types (with possibly similar fields)
                //
                //
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
                        source_field.name.clone().unwrap_or_default(),
                        dest_field.name.clone().unwrap_or_default()
                    )
                })
                .unwrap();

                Some(quote! {
                    #dest_f_name: #struct_mapping,
                })
            }
        }
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
            rodc_util::StructKind::Plain {
                fields: dest_fields,
            } => {
                self.map_struct_plain(source, dest_fields, tokens, dest_path);
            }
        }
    }
}
