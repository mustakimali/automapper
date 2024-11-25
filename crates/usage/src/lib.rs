use models::{DestStruct4, DestStructWithEnum, SourceStruct3, SourceStructWithEnum};

pub mod models;
mod output;
mod protogen;

//
// Structs mappings
automapper::map!(models::SourceStruct => models::DestStruct);
automapper::map!(models::SourceStruct2 => models::DestStruct2);

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
// TODO: Proto enum mappings
automapper::macros::impl_map_fn! {
    fn map_proto_struct(protogen::example::Person => protogen::example::HomoSepiens);
}

// See tests in tests/*.rs folder
