fn __map(value: crate::v2::models::SourceStructWithEnum) -> crate::v2::models::DestStructWithEnum {
    crate::v2::models::DestStructWithEnum {
        enum_: match value.enum_ {
            crate::v2::models::SourceEnumBasic::Unit => crate::v2::models::DestEnumBasic::Unit,
            crate::v2::models::SourceEnumBasic::Touple(item_0, item_1) => {
                crate::v2::models::DestEnumBasic::Touple(item_0, item_1)
            }
            crate::v2::models::SourceEnumBasic::ToupleSingle(item_0) => {
                crate::v2::models::DestEnumBasic::ToupleSingle(item_0)
            }
            crate::v2::models::SourceEnumBasic::Struct {
                field1,
                field2,
                nested,
            } => crate::v2::models::DestEnumBasic::Struct {
                field1: field1,
                field2: field2,
                nested: crate::v2::models::DestStruct {
                    a: nested.a,
                    b: nested.b,
                    s: nested.s,
                },
            },
        },
        field: value.field,
    }
}
