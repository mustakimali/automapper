use models::{DestStruct4, DestStructWithEnum, SourceStruct3, SourceStructWithEnum};

pub mod models;
#[allow(unused, clippy::redundant_field_names)]
mod output;
mod protogen;

//
// Structs mappings
automapper::map!(models::SourceStruct => models::DestStruct);
automapper::map!(models::SourceStruct2 => models::DestStruct2);
automapper::map!(models::SourcePrim => models::DestPrim);

//
// Struct with Optional fields
automapper::map!(SourceStruct3 => DestStruct4);

//
// Enum mappings
automapper::map!(SourceStructWithEnum => DestStructWithEnum);

//
// TODO:Custom mapping
//automapper::map!(models::SourceStructWithDifferentField > models::DestStructWithDifferentField);

//
// Proto mappings
// (Can't use automapper::map! due to orphan rule)
//
automapper::macros::impl_map_fn! {
    fn map_proto_struct(protogen::example::Person => protogen::example::HomoSepiens);
}

// See tests in tests/*.rs folder
