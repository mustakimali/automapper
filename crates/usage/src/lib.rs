use models::{DestStruct4, DestStructWithEnum, SourceStruct3, SourceStructWithEnum};

pub mod gen;
pub mod models;
#[allow(unused, clippy::redundant_field_names)]
mod output;
mod protogen;

//
// Structs mappings
// Basic
automapper::map!(models::SourceStruct => models::DestStruct);
// Recursive mapping of fields
automapper::map!(models::SourceStruct2 => models::DestStruct2);
// Option<T> where T is a struct
automapper::map!(SourceStruct3 => DestStruct4);
// Option<T> where T is primitive
automapper::map!(models::SourcePrim => models::DestPrim);
// Result<T>
automapper::map!(models::SourceStructWithResult => models::DestStructWithResult);
//
// Enum mappings
automapper::map!(SourceStructWithEnum => DestStructWithEnum);

//
// TODO:Custom mapping
//automapper::map!(models::SourceStructWithDifferentField) -> models::DestStructWithDifferentField;
automapper::macros::impl_map_fn! {
    fn map_with_missing_field(models::SourceStructWithDifferentField) -> models::DestStructWithDifferentField (
        a : a,
        b : b,
        s : s,
        missing_field : "default value",
    )
}

//
// Proto mappings
// (Can't use automapper::map! due to orphan rule)
//
automapper::macros::impl_map_fn! {
    fn map_proto_struct(protogen::example::Person) -> protogen::example::HomoSepiens;
}

// See tests in tests/*.rs folder
