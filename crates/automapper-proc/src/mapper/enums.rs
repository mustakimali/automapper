use std::ops::ControlFlow;

use anyhow::Context;
use quote::{format_ident, quote, ToTokens};
use rustdoc_types::{GenericArg, GenericArgs};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, FieldKind, KindAsStr, RustType, StructRustType},
    TypeToTypeMapping,
};

impl TypeToTypeMapping {
    /// Builds the token stream for the mapping of the source enum variant to the destination enum variant
    /// ```ignore
    ///    SomeEnum::Variant1[..] => DestEnum::Variant1[..],
    /// ```
    pub(crate) fn create_single_enum_variant_mapping(
        &self,
        target_enum: &rodc_util::EnumRustType,
        source_variant: &rodc_util::EnumVariant,
    ) -> proc_macro2::TokenStream {
        let Some(matching_dest_v) = target_enum
            .variants
            .iter()
            .find(|v| v.name == source_variant.name)
        else {
            panic!(
                "failed to find matching source variant for dest enum field: {}",
                source_variant.name.clone()
            );
        };

        if !source_variant.are_same_kind(matching_dest_v) {
            panic!(
                "source and dest variant kind mismatch: source: {}, dest: {}",
                source_variant.kind_as_str(),
                matching_dest_v.kind_as_str()
            );
        }

        let source_v_name = format_ident!("{}", source_variant.name.clone());
        let dest_v_name = format_ident!("{}", matching_dest_v.name.clone());
        let source_path = self.source.path();
        let dest_path = self.dest.path();

        match &source_variant.kind {
            rodc_util::EnumVariantKind::Plain => {
                let rodc_util::EnumVariantKind::Plain = matching_dest_v.kind else {
                    unreachable!() // same kind check happened above
                };

                quote! {
                    #source_path::#source_v_name => #dest_path::#dest_v_name,

                }
            }
            rodc_util::EnumVariantKind::Tuple(source_v_tuple_items) => {
                let rodc_util::EnumVariantKind::Tuple(_dest_v_tuple_items) = &matching_dest_v.kind
                // TODO: allow touple items in different order
                else {
                    unreachable!() // same kind check happened above
                };

                let source_items_i = source_v_tuple_items
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format_ident!("item_{}", i))
                    .collect::<Vec<_>>();

                // TODO: verify data types (and perform recursive mapping if needed)
                quote! {
                    #source_path::#source_v_name(#(#source_items_i),*) => #dest_path::#dest_v_name(#(#source_items_i),*),
                }
            }
            rodc_util::EnumVariantKind::Struct {
                fields: source_v_fields,
            } => {
                let rodc_util::EnumVariantKind::Struct {
                    fields: dest_v_fields,
                } = &matching_dest_v.kind
                else {
                    unreachable!() // same kind check happened above
                };

                let fields_i = source_v_fields
                    .iter()
                    .map(|f| format_ident!("{}", f.name.clone().expect("name for struct field")))
                    .collect::<Vec<_>>();

                let mut variant_field_mappings = Vec::with_capacity(dest_v_fields.len());

                for dest_v_field in dest_v_fields {
                    let mapping = self
                        .create_struct_field_mapping(
                            source_v_fields,
                            dest_v_field,
                            None, // just use the field name
                        )
                        .unwrap();

                    variant_field_mappings.push(mapping);
                }

                quote! {
                    #source_path::#source_v_name{ #(#fields_i),* } => #dest_path::#dest_v_name { #(#variant_field_mappings)* },
                }
            }
        }
    }
}
