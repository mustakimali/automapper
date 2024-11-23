use std::ops::ControlFlow;

use anyhow::Context;
use quote::{format_ident, quote, ToTokens};
use rustdoc_types::{GenericArg, GenericArgs};

use crate::{
    models::context::MacroCtx,
    rodc_util::{self, KindAsStr, RustType, StructFieldKind, StructRustType},
};

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
        let source = rodc_util::find_types_try_exact(&source_path, &ctx.rdocs)
            .with_context(|| {
                format!(
                    "failed to find source type: `{}`",
                    source_path.to_token_stream()
                )
            })
            .unwrap();

        let dest = rodc_util::find_types_try_exact(&dest_path, &ctx.rdocs)
            .with_context(|| {
                format!(
                    "failed to find dest type: `{}`",
                    dest_path.to_token_stream()
                )
            })
            .unwrap();

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

        let accessor = self.source_field_accessor();

        let mut mappings = Vec::with_capacity(dest_fields.len());
        for dest_f in dest_fields.iter() {
            let Some(map) = self.create_field_mapping(source_fields, dest_f, Some(&accessor))
            else {
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
        accessor: Option<&proc_macro2::TokenStream>,
    ) -> Option<proc_macro2::TokenStream> {
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

        match &dest_field.kind {
            rodc_util::StructFieldKind::Primitive { name: _ } => {
                if dest_field.kind.is_primitive_eq(&source_field.kind) {
                    // primitive types: can be directly assigned

                    Some(quote! {
                        #dest_f_name: #accessor_with_field, /* primative type */
                    })
                } else {
                    // primitive types: may require explicit casting
                    //TODO(FIX): only castable types
                    Some(quote! {
                        #dest_f_name: #accessor_with_field as _, /* primative type with casting */
                    })
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
                    let source_t_of_option = source_field.t_of_option().unwrap();
                    let dest_t_of_option = dest_field.t_of_option().unwrap();

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
                    })
                    .unwrap();

                    return Some(quote! {
                        #dest_f_name: #accessor.#source_f_name.map(|v| {
                            #struct_mapping_inside_lambda
                        }),
                    });
                }

                // TODO: Result<T, E> mapping

                // Possiblity: Same type with different generic arg
                //
                //
                // TODO: error

                // Possiblity: Same type of struct to struct mapping (non generic)
                //
                if dest_path.id == source_path.id {
                    // same path: can be directly assigned
                    return Some(quote! {
                        #dest_f_name: #accessor_with_field, /* primative type */
                    });
                }

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
                .unwrap();

                Some(quote! {
                    #dest_f_name: #nested_type_mappings,
                })
            }
        }
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
                    rodc_util::StructKind::Tuple(_vec) => todo!(),
                    rodc_util::StructKind::Plain {
                        fields: dest_fields,
                    } => {
                        self.map_struct_plain(source, dest_fields, tokens, dest_path);
                    }
                }
            }
            RustType::Enum(dest_enum) => {
                let RustType::Enum(source) = &self.source else {
                    unreachable!()
                };

                let accessor = self.source_field_accessor();

                // TODO: handle non-exhaustive enum
                let mut mappings = Vec::with_capacity(source.variants.len());

                for source_v in &source.variants {
                    let Some(matching_dest_v) =
                        dest_enum.variants.iter().find(|v| v.name == source_v.name)
                    else {
                        panic!(
                            "failed to find matching source variant for dest enum field: {}",
                            source_v.name.clone()
                        );
                    };

                    if !source_v.are_same_kind(&matching_dest_v) {
                        panic!(
                            "source and dest variant kind mismatch: source: {}, dest: {}",
                            source_v.kind_as_str(),
                            matching_dest_v.kind_as_str()
                        );
                    }

                    let source_v_name = format_ident!("{}", source_v.name.clone());
                    let dest_v_name = format_ident!("{}", matching_dest_v.name.clone());
                    let source_path = self.source.path();
                    let dest_path = self.dest.path();

                    match &source_v.kind {
                        rodc_util::EnumVariantKind::Plain => {
                            let rodc_util::EnumVariantKind::Plain = matching_dest_v.kind else {
                                unreachable!() // same kind check happened above
                            };

                            mappings.push(quote! {
                                #source_path::#source_v_name => #dest_path::#dest_v_name,

                            });
                        }
                        rodc_util::EnumVariantKind::Tuple(source_v_tuple_items) => {
                            let rodc_util::EnumVariantKind::Tuple(dest_v_tuple_items) =
                                &matching_dest_v.kind
                            else {
                                unreachable!() // same kind check happened above
                            };

                            let source_items_i = source_v_tuple_items
                                .iter()
                                .enumerate()
                                .map(|(i, _)| format_ident!("item_{}", i))
                                .collect::<Vec<_>>();

                            mappings.push(quote! {
                                #source_path::#source_v_name(#(#source_items_i),*) => #dest_path::#dest_v_name(#(#source_items_i),*),
                            });
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
                                .map(|f| {
                                    format_ident!(
                                        "{}",
                                        f.name.clone().expect("name for struct field")
                                    )
                                })
                                .collect::<Vec<_>>();

                            let mut variant_field_mappings =
                                Vec::with_capacity(dest_v_fields.len());

                            for dest_v_field in dest_v_fields {
                                let Some(mapping) = self.create_field_mapping(
                                    &source_v_fields,
                                    dest_v_field,
                                    None, // just use the field name
                                ) else {
                                    continue;
                                };

                                variant_field_mappings.push(mapping);
                            }

                            mappings.push(quote! {
                                #source_path::#source_v_name{ #(#fields_i),* } => #dest_path::#dest_v_name { #(#variant_field_mappings)* },
                            });
                        }
                    }
                }

                tokens.extend(quote! {
                    match #accessor {
                        #(#mappings)*
                    }
                })
            }
        }
    }
}
