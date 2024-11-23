use anyhow::Context;
use quote::{format_ident, quote, ToTokens};
use rustdoc_types::{GenericArg, GenericArgs};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, KindAsStr, RustType, StructFieldKind, StructRustType},
};

mod enums;
mod structs;

pub struct TypeToTypeMapping {
    pub source: RustType,
    /// The path to the accessor function for the source struct.
    /// Starting from `value` in the root mapping function.
    source_field_accessor: Vec<String>,
    pub dest: RustType,
    ctx: MacroCtx,
}

impl TypeToTypeMapping {
    pub fn new(
        source_path: syn::Path,
        source_accessor: Vec<String>,
        dest_path: syn::Path,
        ctx: MacroCtx,
    ) -> anyhow::Result<Self> {
        let source =
            rodc_util::find_types_try_exact(&source_path, &ctx.rdocs).with_context(|| {
                format!(
                    "failed to find source type: `{}`",
                    source_path.to_token_stream()
                )
            })?;

        let dest = rodc_util::find_types_try_exact(&dest_path, &ctx.rdocs).with_context(|| {
            format!(
                "failed to find dest type: `{}`",
                dest_path.to_token_stream()
            )
        })?;

        anyhow::ensure!(
            source.are_same_kind(&dest),
            "don't know how to map between structs and enums"
        );

        Ok(Self {
            source,
            source_field_accessor: source_accessor,
            dest,
            ctx,
        })
    }

    /// Convert the accessor (how to access the source field being mapped / the current field being mapped)
    /// into a token stream
    fn source_field_accessor(&self) -> proc_macro2::TokenStream {
        let accessor = self
            .source_field_accessor
            .iter()
            .map(|i| format_ident!("{}", i))
            .collect::<Vec<_>>();
        quote! { #(#accessor).* }
    }
}

impl ToTokens for TypeToTypeMapping {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let dest = &self.dest;
        let dest_path = dest.path();

        match dest {
            RustType::Struct(dest_struct) => {
                let RustType::Struct(source) = &self.source else {
                    unreachable!()
                };

                match &dest_struct.kind {
                    rodc_util::StructKind::Unit => {
                        tokens.extend(quote! {
                            #dest_path // unit struct
                        });
                    }
                    rodc_util::StructKind::Tuple(_vec) => {
                        unimplemented!("tuple struct mapping")
                    }
                    rodc_util::StructKind::Plain {
                        fields: dest_fields,
                    } => {
                        let token = self
                            .create_struct_mapping_plain(source, dest_fields, dest_path)
                            .expect("map_struct_plain");
                        tokens.extend(token);
                    }
                }
            }
            RustType::Enum(dest_enum) => {
                let RustType::Enum(source) = &self.source else {
                    unreachable!()
                };

                let accessor = self.source_field_accessor();

                // TODO: handle non-exhaustive enum
                let mut variant_mappings = Vec::with_capacity(source.variants.len());

                for source_v in &source.variants {
                    let token = self.create_single_enum_variant_mapping(dest_enum, source_v);
                    variant_mappings.push(token);
                }

                tokens.extend(quote! {
                    match #accessor {
                        #(#variant_mappings)*
                    }
                })
            }
        }
    }
}
