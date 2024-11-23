use anyhow::Context;
use quote::{format_ident, quote, ToTokens};
use rustdoc_types::{GenericArg, GenericArgs};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, KindAsStr, RustType, StructFieldKind, StructRustType},
    TypeToTypeMapping,
};

impl TypeToTypeMapping {
    pub(crate) fn create_struct_mapping_plain(
        &self,
        source: &StructRustType,
        dest_fields: &[rodc_util::StructFieldOrEnumVariant],
        dest_path: syn::Path,
    ) -> anyhow::Result<proc_macro2::TokenStream> {
        let rodc_util::StructKind::Plain {
            fields: source_fields,
        } = &source.kind
        else {
            panic!("source struct is not plain struct");
        };

        let accessor = self.source_field_accessor();

        let mappings = dest_fields
            .iter()
            .map(|dest_f| self.create_struct_field_mapping(source_fields, dest_f, Some(&accessor)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(quote! {
            #dest_path {
                #(#mappings)*
            }
        })
    }

    pub(crate) fn create_struct_field_mapping(
        &self,
        source_fields: &[rodc_util::StructFieldOrEnumVariant],
        dest_field: &rodc_util::StructFieldOrEnumVariant,
        accessor: Option<&proc_macro2::TokenStream>,
    ) -> anyhow::Result<proc_macro2::TokenStream> {
        // ^ TODO: remove Option
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

        let accessor_with_field = match accessor {
            Some(accessor) => quote! {
                #accessor.#source_f_name
            },
            None => quote! { #source_f_name },
        };

        let token_stream = match &dest_field.kind {
            rodc_util::StructFieldKind::Primitive { name: _ } => {
                if dest_field.kind.is_primitive_eq(&source_field.kind) {
                    // primitive types: can be directly assigned
                    quote! {
                        #dest_f_name: #accessor_with_field, /* primative type */
                    }
                } else {
                    // primitive types: may require explicit casting
                    //TODO(FIX): only castable types
                    quote! {
                        #dest_f_name: #accessor_with_field as _, /* primative type with casting */
                    }
                }
            }
            rodc_util::StructFieldKind::ResolvedPath { path: dest_path } => {
                let rodc_util::StructFieldKind::ResolvedPath { path: source_path } =
                    &source_field.kind
                else {
                    unreachable!("must be resolved path")
                };

                // Possiblity: Option<T>
                //
                //
                if StructFieldKind::are_both_option_type(&source_field.kind, &dest_field.kind) {
                    let source_t_of_option = source_field.t_of_option()?;
                    let dest_t_of_option = dest_field.t_of_option()?;

                    let struct_mapping_inside_lambda = TypeToTypeMapping::new(
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
                    })?;

                    quote! {
                        #dest_f_name: #accessor.#source_f_name.map(|v| {
                            #struct_mapping_inside_lambda
                        }),
                    }
                }
                // TODO: Result<T, E> mapping

                // Possiblity: Same type with different generic arg
                //
                //
                // TODO: error

                // Possiblity: Same type of struct to struct mapping (non generic)
                //
                else if dest_path.id == source_path.id {
                    // same path: can be directly assigned
                    quote! {
                        #dest_f_name: #accessor_with_field, /* same type */
                    }
                } else {
                    // Possiblity: Other types (with possibly similar fields)
                    //
                    //
                    let new_source_accessor = {
                        let mut s = accessor
                            .map(|_| self.source_field_accessor.clone())
                            .unwrap_or_default();
                        s.push(source_f_name.to_string());
                        s
                    };

                    let nested_type_mappings = TypeToTypeMapping::new(
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
                    .with_context(|| {
                        format!("Mapping {} to {}", source_path.name, dest_path.name)
                    })?;

                    quote! {
                        #dest_f_name: #nested_type_mappings,
                    }
                }
            }
        };

        Ok(token_stream)
    }
}
