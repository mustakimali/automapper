
pub mod mod_1733761603897_q17hk95a7by51f {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1733761603907_xr0pvh0fdzmdrp {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1733761603964_qfvxm2nn21n8md {
    fn __map(value: crate::models::SourceStruct2) -> crate::models::DestStruct2 {
        crate::models::DestStruct2 {
            s: value.s,
            nested: crate::models::DestStruct {
                a: value.nested.a,
                b: value.nested.b,
                s: value.nested.s,
            },
        }
    }
}

pub mod mod_1733761603975_z9gd2ka1e7xvte {
    fn __map(value: crate::models::SourceStruct2) -> crate::models::DestStruct2 {
        crate::models::DestStruct2 {
            s: value.s,
            nested: crate::models::DestStruct {
                a: value.nested.a,
                b: value.nested.b,
                s: value.nested.s,
            },
        }
    }
}

pub mod mod_1733761604035_0h4x5891wr6pk8 {
    fn __map(value: crate::models::SourceStruct3) -> crate::models::DestStruct4 {
        crate::models::DestStruct4 {
            s: value.s,
            nested: crate::models::DestStruct {
                a: value.nested.a,
                b: value.nested.b,
                s: value.nested.s,
            },
            optional: value.optional.map(|v| crate::models::DestStruct {
                a: v.a,
                b: v.b,
                s: v.s,
            }),
        }
    }
}

pub mod mod_1733761604044_qkp39ejvftea3a {
    fn __map(value: crate::models::SourceStruct3) -> crate::models::DestStruct4 {
        crate::models::DestStruct4 {
            s: value.s,
            nested: crate::models::DestStruct {
                a: value.nested.a,
                b: value.nested.b,
                s: value.nested.s,
            },
            optional: value.optional.map(|v| crate::models::DestStruct {
                a: v.a,
                b: v.b,
                s: v.s,
            }),
        }
    }
}

pub mod mod_1733761604095_gy5pg2qqddfh7t {
    fn __map(value: crate::models::SourcePrim) -> crate::models::DestPrim {
        crate::models::DestPrim { a: value.a }
    }
}

pub mod mod_1733761604107_rhe2hqyx5030mm {
    fn __map(value: crate::models::SourcePrim) -> crate::models::DestPrim {
        crate::models::DestPrim { a: value.a }
    }
}

pub mod mod_1733761604162_dqxzdf7y5t01kf {
    fn __map(value: crate::models::SourceStructWithResult) -> crate::models::DestStructWithResult {
        crate::models::DestStructWithResult {
            field: value.field.map(|v| crate::models::DestStruct2 {
                s: v.s,
                nested: crate::models::DestStruct {
                    a: v.nested.a,
                    b: v.nested.b,
                    s: v.nested.s,
                },
            }),
        }
    }
}

pub mod mod_1733761604179_tbyh7gshqjrqhc {
    fn __map(value: crate::models::SourceStructWithResult) -> crate::models::DestStructWithResult {
        crate::models::DestStructWithResult {
            field: value.field.map(|v| crate::models::DestStruct2 {
                s: v.s,
                nested: crate::models::DestStruct {
                    a: v.nested.a,
                    b: v.nested.b,
                    s: v.nested.s,
                },
            }),
        }
    }
}

pub mod mod_1733761604232_vqhf3c0v41h9j2 {
    fn __map(value: crate::models::SourceStructWithEnum) -> crate::models::DestStructWithEnum {
        crate::models::DestStructWithEnum {
            enum_: match value.enum_ {
                crate::models::SourceEnumBasic::Unit => crate::models::DestEnumBasic::Unit,
                crate::models::SourceEnumBasic::Touple(item_0, item_1) => {
                    crate::models::DestEnumBasic::Touple(item_0, item_1)
                }
                crate::models::SourceEnumBasic::ToupleSingle(item_0) => {
                    crate::models::DestEnumBasic::ToupleSingle(item_0)
                }
                crate::models::SourceEnumBasic::Struct {
                    field1,
                    field2,
                    nested,
                } => crate::models::DestEnumBasic::Struct {
                    field1: field1,
                    field2: field2,
                    nested: crate::models::DestStruct {
                        a: nested.a,
                        b: nested.b,
                        s: nested.s,
                    },
                },
            },
            field: value.field,
        }
    }
}

pub mod mod_1733761604244_2wg1z0r25d1rsm {
    fn __map(value: crate::models::SourceStructWithEnum) -> crate::models::DestStructWithEnum {
        crate::models::DestStructWithEnum {
            enum_: match value.enum_ {
                crate::models::SourceEnumBasic::Unit => crate::models::DestEnumBasic::Unit,
                crate::models::SourceEnumBasic::Touple(item_0, item_1) => {
                    crate::models::DestEnumBasic::Touple(item_0, item_1)
                }
                crate::models::SourceEnumBasic::ToupleSingle(item_0) => {
                    crate::models::DestEnumBasic::ToupleSingle(item_0)
                }
                crate::models::SourceEnumBasic::Struct {
                    field1,
                    field2,
                    nested,
                } => crate::models::DestEnumBasic::Struct {
                    field1: field1,
                    field2: field2,
                    nested: crate::models::DestStruct {
                        a: nested.a,
                        b: nested.b,
                        s: nested.s,
                    },
                },
            },
            field: value.field,
        }
    }
}

pub mod mod_1733761604294_ywvwjxpyrrszeq {
    fn map_with_missing_field(
        value: crate::models::SourceStructWithDifferentField,
    ) -> crate::models::DestStructWithDifferentField {
        crate::models::DestStructWithDifferentField {
            a: value.a,
            b: value.b,
            c: value.z,
        }
    }
}

pub mod mod_1733761604308_f874f7fqkxbkgv {
    fn map_with_missing_field(
        value: crate::models::SourceStructWithDifferentField,
    ) -> crate::models::DestStructWithDifferentField {
        crate::models::DestStructWithDifferentField {
            a: value.a,
            b: value.b,
            c: value.z,
        }
    }
}

pub mod mod_1733761604360_5ytnpfyhraw92s {
    fn map_proto_struct(
        value: crate::protogen::example::Person,
    ) -> crate::protogen::example::HomoSepiens {
        crate::protogen::example::HomoSepiens {
            first_name: value.first_name,
            last_name: value.last_name,
            gender: value
                .gender
                .map(|v| crate::protogen::example::HomoSepiensGender {
                    gender: v.gender.map(|v| match v {
                        crate::protogen::example::gender::Gender::Male(item_0) => {
                            crate::protogen::example::homo_sepiens_gender::Gender::Male(item_0)
                        }
                        crate::protogen::example::gender::Gender::Female(item_0) => {
                            crate::protogen::example::homo_sepiens_gender::Gender::Female(item_0)
                        }
                    }),
                }),
        }
    }
}

pub mod mod_1733761604376_gngxws7ekze4sw {
    fn map_proto_struct(
        value: crate::protogen::example::Person,
    ) -> crate::protogen::example::HomoSepiens {
        crate::protogen::example::HomoSepiens {
            first_name: value.first_name,
            last_name: value.last_name,
            gender: value
                .gender
                .map(|v| crate::protogen::example::HomoSepiensGender {
                    gender: v.gender.map(|v| match v {
                        crate::protogen::example::gender::Gender::Male(item_0) => {
                            crate::protogen::example::homo_sepiens_gender::Gender::Male(item_0)
                        }
                        crate::protogen::example::gender::Gender::Female(item_0) => {
                            crate::protogen::example::homo_sepiens_gender::Gender::Female(item_0)
                        }
                    }),
                }),
        }
    }
}
