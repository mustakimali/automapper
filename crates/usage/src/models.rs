#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceStruct {
    pub a: i32,
    pub b: u32,
    pub s: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestStruct {
    pub a: i32,
    pub b: u32,
    pub s: String,
}

//
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceStruct2 {
    pub s: String,
    pub nested: SourceStruct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestStruct2 {
    pub s: String,
    pub nested: DestStruct,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestStruct3 {
    pub s: String,
    pub nested: DestStruct,
}

//
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceStruct3 {
    pub s: String,
    pub nested: SourceStruct,
    pub optional: Option<SourceStruct>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestStruct4 {
    pub s: String,
    pub nested: DestStruct,
    pub optional: Option<DestStruct>,
}

#[derive(Debug, Clone)]
pub struct SourcePrim {
    pub a: Option<u32>,
}
#[derive(Debug, Clone)]
pub struct DestPrim {
    pub a: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceStructWithEnum {
    pub enum_: SourceEnumBasic,
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestStructWithEnum {
    pub enum_: DestEnumBasic, // different enum
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceEnumBasic {
    Unit,
    Touple(u32, u32),
    ToupleSingle(SourceStruct),
    Struct {
        field1: u32,
        field2: String,
        nested: SourceStruct,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestEnumBasic {
    Unit,
    Touple(u32, u32),
    ToupleSingle(SourceStruct),
    Struct {
        field1: u32,
        field2: String,
        nested: DestStruct,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceStructWithDifferentField {
    pub a: i32,
    pub b: i32,
    pub z: i32, // c is missing
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestStructWithDifferentField {
    pub a: i32,
    pub b: i32,
    pub c: i32, // this needs to be mapped manually
}
