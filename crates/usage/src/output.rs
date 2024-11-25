
pub mod mod_1732566036268_tf84qp5kph5pgd {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1732566036270_2baxxx1y2abrnx {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1732566036321_f16p86pmnryv3b {
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

pub mod mod_1732566036324_1sxb89xgbpjkdv {
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

pub mod mod_1732566036374_wa2v4mbp64kaz2 {
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

pub mod mod_1732566036377_vzhd541d4qzvqe {
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

pub mod mod_1732566036425_2qcgfa9v825r91 {
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

pub mod mod_1732566036431_fr9fk6515jvhjz {
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

pub mod mod_1732566036480_xams6kswvpvczb {
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

pub mod mod_1732566036485_j5epc0qpcaw2z7 {
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
