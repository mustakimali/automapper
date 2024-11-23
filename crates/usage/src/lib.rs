use models::{DestStruct4, DestStructWithEnum, SourceStruct3, SourceStructWithEnum};

pub mod models;

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
// Custom mapping
//automapper::map!(models::SourceStructWithDifferentField > models::DestStructWithDifferentField);

// See tests in tests/*.rs folder
