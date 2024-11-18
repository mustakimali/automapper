#[derive(Debug, Clone)]
pub struct SourceStruct {
    pub a: i32,
    pub b: u32,
    pub s: String,
}

#[derive(Debug, Clone)]
pub struct DestStruct {
    pub a: i32,
    pub b: u32,
    pub s: String,
}

//
//
//

#[derive(Debug, Clone)]
pub struct SourceStruct2 {
    pub s: String,
    pub nested: SourceStruct,
}

#[derive(Debug, Clone)]
pub struct DestStruct2 {
    pub s: String,
    pub nested: DestStruct,
}
#[derive(Debug, Clone)]
pub struct DestStruct3 {
    pub s: String,
    pub nested: DestStruct,
}

//
//
//

#[derive(Debug, Clone)]
pub struct SourceStruct3 {
    pub s: String,
    pub nested: SourceStruct,
    pub optional: Option<SourceStruct>,
}

#[derive(Debug, Clone)]
pub struct DestStruct4 {
    pub s: String,
    pub nested: DestStruct,
    pub optional: Option<DestStruct>,
}
