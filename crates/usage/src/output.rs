
pub mod mod_1733505064004_qa377ztyjbs6jv {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1733505064006_ynchp4xjfzscjy {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1733505064066_8161fpg38t11vb {
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

pub mod mod_1733505064070_rf42zjy9gsv1dg {
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

pub mod mod_1733505064127_w84jx3x0msg7qr {
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

pub mod mod_1733505064130_akvhnb552471jh {
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

pub mod mod_1733505064187_xkqmymappca12a {
    fn __map(value: crate::models::SourcePrim) -> crate::models::DestPrim {
        crate::models::DestPrim { a: value.a }
    }
}

pub mod mod_1733505064190_wqfdr0whcc6k04 {
    fn __map(value: crate::models::SourcePrim) -> crate::models::DestPrim {
        crate::models::DestPrim { a: value.a }
    }
}

pub mod mod_1733505064245_npfrgbysnvaysb {
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

pub mod mod_1733505064248_qnqy20qna36tet {
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

pub mod mod_1733505064303_aqy6j81vm5vgct {
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

pub mod mod_1733505064306_p8jww1ey87dt1v {
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

pub mod mod_1733505064362_gf60qxb92554kg {
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

pub mod mod_1733505064366_hpt3zjkdm46zcg {
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

pub mod mod_1733505066988_m99xsv39ssy4rj {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1733505066988_q5h1ax7xn2k392 {
    fn __map(value: crate::models::SourceStruct) -> crate::models::DestStruct {
        crate::models::DestStruct {
            a: value.a,
            b: value.b,
            s: value.s,
        }
    }
}

pub mod mod_1733505067050_53c5t61e6vhfff {
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

pub mod mod_1733505067050_xpq6395jdxxndc {
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

pub mod mod_1733505067110_hqm0q7t1j711ns {
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

pub mod mod_1733505067110_9s25eseker2a7t {
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

pub mod mod_1733505067169_k80cmfabntz76j {
    fn __map(value: crate::models::SourcePrim) -> crate::models::DestPrim {
        crate::models::DestPrim { a: value.a }
    }
}

pub mod mod_1733505067173_23eweg0w9rb3e7 {
    fn __map(value: crate::models::SourcePrim) -> crate::models::DestPrim {
        crate::models::DestPrim { a: value.a }
    }
}

pub mod mod_1733505067227_ztrnyr7k2p3nc1 {
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

pub mod mod_1733505067239_v9eqknhzyhhbkx {
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

pub mod mod_1733505067288_tdxrxv266g86qd {
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

pub mod mod_1733505067302_0xdnpefd23t131 {
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

pub mod mod_1733505067348_y0ebvf985fjb83 {
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

pub mod mod_1733505067364_pxk3j6ddxnkdca {
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


        pub mod mod_1733505071719_5ajev9znbq9zzm {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733505071721_91t2zw9awjsq1z {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733505071778_sfvhz2bbz7mrd5 {
            fn __map(value : crate :: models :: SourceStruct2) -> crate :: models ::
DestStruct2
{
    crate :: models :: DestStruct2
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
    }
}
        }

        pub mod mod_1733505071780_bd8v0hycxfppme {
            fn __map(value : crate :: models :: SourceStruct2) -> crate :: models ::
DestStruct2
{
    crate :: models :: DestStruct2
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
    }
}
        }

        pub mod mod_1733505071837_xhy7s3raepfxn0 {
            fn __map(value : crate :: models :: SourceStruct3) -> crate :: models ::
DestStruct4
{
    crate :: models :: DestStruct4
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        { crate :: models :: DestStruct { a : v.a, b : v.b, s : v.s, } }),
    }
}
        }

        pub mod mod_1733505071840_r73m10bd3834be {
            fn __map(value : crate :: models :: SourceStruct3) -> crate :: models ::
DestStruct4
{
    crate :: models :: DestStruct4
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        { crate :: models :: DestStruct { a : v.a, b : v.b, s : v.s, } }),
    }
}
        }

        pub mod mod_1733505071895_mdb3vfrs9zt5rx {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733505071900_vgm61em3pnv0tz {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733505071957_bdagqenw6nkpzf {
            fn __map(value : crate :: models :: SourceStructWithResult) -> crate :: models
:: DestStructWithResult
{
    crate :: models :: DestStructWithResult
    {
        field :
        value.field.map(| v |
        {
            crate :: models :: DestStruct2
            {
                s : v.s, nested : crate :: models :: DestStruct
                { a : v.nested.a, b : v.nested.b, s : v.nested.s, },
            }
        }),
    }
}
        }

        pub mod mod_1733505071962_skqrv2a9btfnx9 {
            fn __map(value : crate :: models :: SourceStructWithResult) -> crate :: models
:: DestStructWithResult
{
    crate :: models :: DestStructWithResult
    {
        field :
        value.field.map(| v |
        {
            crate :: models :: DestStruct2
            {
                s : v.s, nested : crate :: models :: DestStruct
                { a : v.nested.a, b : v.nested.b, s : v.nested.s, },
            }
        }),
    }
}
        }

        pub mod mod_1733505072018_w7fmh3vq07yaby {
            fn __map(value : crate :: models :: SourceStructWithEnum) -> crate :: models
:: DestStructWithEnum
{
    crate :: models :: DestStructWithEnum
    {
        enum_ : match value.enum_
        {
            crate :: models :: SourceEnumBasic :: Unit => crate :: models ::
            DestEnumBasic :: Unit, crate :: models :: SourceEnumBasic ::
            Touple(item_0, item_1) => crate :: models :: DestEnumBasic ::
            Touple(item_0, item_1), crate :: models :: SourceEnumBasic ::
            ToupleSingle(item_0) => crate :: models :: DestEnumBasic ::
            ToupleSingle(item_0), crate :: models :: SourceEnumBasic :: Struct
            { field1, field2, nested } => crate :: models :: DestEnumBasic ::
            Struct
            {
                field1 : field1, field2 : field2, nested : crate :: models ::
                DestStruct { a : nested.a, b : nested.b, s : nested.s, },
            },
        }, field : value.field,
    }
}
        }

        pub mod mod_1733505072025_evqybmkes3270x {
            fn __map(value : crate :: models :: SourceStructWithEnum) -> crate :: models
:: DestStructWithEnum
{
    crate :: models :: DestStructWithEnum
    {
        enum_ : match value.enum_
        {
            crate :: models :: SourceEnumBasic :: Unit => crate :: models ::
            DestEnumBasic :: Unit, crate :: models :: SourceEnumBasic ::
            Touple(item_0, item_1) => crate :: models :: DestEnumBasic ::
            Touple(item_0, item_1), crate :: models :: SourceEnumBasic ::
            ToupleSingle(item_0) => crate :: models :: DestEnumBasic ::
            ToupleSingle(item_0), crate :: models :: SourceEnumBasic :: Struct
            { field1, field2, nested } => crate :: models :: DestEnumBasic ::
            Struct
            {
                field1 : field1, field2 : field2, nested : crate :: models ::
                DestStruct { a : nested.a, b : nested.b, s : nested.s, },
            },
        }, field : value.field,
    }
}
        }

        pub mod mod_1733505072080_bx8c67vftavpeb {
            fn map_proto_struct(value : crate :: protogen :: example :: Person) -> crate
:: protogen :: example :: HomoSepiens
{
    crate :: protogen :: example :: HomoSepiens
    {
        first_name : value.first_name, last_name : value.last_name, gender :
        value.gender.map(| v |
        {
            crate :: protogen :: example :: HomoSepiensGender
            {
                gender :
                v.gender.map(| v |
                {
                    match v
                    {
                        crate :: protogen :: example :: gender :: Gender ::
                        Male(item_0) => crate :: protogen :: example ::
                        homo_sepiens_gender :: Gender :: Male(item_0), crate ::
                        protogen :: example :: gender :: Gender :: Female(item_0) =>
                        crate :: protogen :: example :: homo_sepiens_gender ::
                        Gender :: Female(item_0),
                    }
                }),
            }
        }),
    }
}
        }

        pub mod mod_1733505072090_v1cawcpzpdn8ht {
            fn map_proto_struct(value : crate :: protogen :: example :: Person) -> crate
:: protogen :: example :: HomoSepiens
{
    crate :: protogen :: example :: HomoSepiens
    {
        first_name : value.first_name, last_name : value.last_name, gender :
        value.gender.map(| v |
        {
            crate :: protogen :: example :: HomoSepiensGender
            {
                gender :
                v.gender.map(| v |
                {
                    match v
                    {
                        crate :: protogen :: example :: gender :: Gender ::
                        Male(item_0) => crate :: protogen :: example ::
                        homo_sepiens_gender :: Gender :: Male(item_0), crate ::
                        protogen :: example :: gender :: Gender :: Female(item_0) =>
                        crate :: protogen :: example :: homo_sepiens_gender ::
                        Gender :: Female(item_0),
                    }
                }),
            }
        }),
    }
}
        }

        pub mod mod_1733505074026_th8s3j3e7xv0fp {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733505074027_txrj010hkxd74r {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733505074089_8e6w00cq0cfgcn {
            fn __map(value : crate :: models :: SourceStruct2) -> crate :: models ::
DestStruct2
{
    crate :: models :: DestStruct2
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
    }
}
        }

        pub mod mod_1733505074090_hfs1x4v50qwpet {
            fn __map(value : crate :: models :: SourceStruct2) -> crate :: models ::
DestStruct2
{
    crate :: models :: DestStruct2
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
    }
}
        }

        pub mod mod_1733505074150_vrgdqm2wcs05nj {
            fn __map(value : crate :: models :: SourceStruct3) -> crate :: models ::
DestStruct4
{
    crate :: models :: DestStruct4
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        { crate :: models :: DestStruct { a : v.a, b : v.b, s : v.s, } }),
    }
}
        }

        pub mod mod_1733505074150_5d2yda3px3gkay {
            fn __map(value : crate :: models :: SourceStruct3) -> crate :: models ::
DestStruct4
{
    crate :: models :: DestStruct4
    {
        s : value.s, nested : crate :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        { crate :: models :: DestStruct { a : v.a, b : v.b, s : v.s, } }),
    }
}
        }

        pub mod mod_1733505074211_vbgrvhwg6b1yrn {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733505074211_qnr89kjcqx7s6x {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733505074270_18acf0tkmj8rhx {
            fn __map(value : crate :: models :: SourceStructWithResult) -> crate :: models
:: DestStructWithResult
{
    crate :: models :: DestStructWithResult
    {
        field :
        value.field.map(| v |
        {
            crate :: models :: DestStruct2
            {
                s : v.s, nested : crate :: models :: DestStruct
                { a : v.nested.a, b : v.nested.b, s : v.nested.s, },
            }
        }),
    }
}
        }

        pub mod mod_1733505074270_ewbk0vyp2sgt3e {
            fn __map(value : crate :: models :: SourceStructWithResult) -> crate :: models
:: DestStructWithResult
{
    crate :: models :: DestStructWithResult
    {
        field :
        value.field.map(| v |
        {
            crate :: models :: DestStruct2
            {
                s : v.s, nested : crate :: models :: DestStruct
                { a : v.nested.a, b : v.nested.b, s : v.nested.s, },
            }
        }),
    }
}
        }

        pub mod mod_1733505074333_ayn2m8bhf0syxf {
            fn __map(value : crate :: models :: SourceStructWithEnum) -> crate :: models
:: DestStructWithEnum
{
    crate :: models :: DestStructWithEnum
    {
        enum_ : match value.enum_
        {
            crate :: models :: SourceEnumBasic :: Unit => crate :: models ::
            DestEnumBasic :: Unit, crate :: models :: SourceEnumBasic ::
            Touple(item_0, item_1) => crate :: models :: DestEnumBasic ::
            Touple(item_0, item_1), crate :: models :: SourceEnumBasic ::
            ToupleSingle(item_0) => crate :: models :: DestEnumBasic ::
            ToupleSingle(item_0), crate :: models :: SourceEnumBasic :: Struct
            { field1, field2, nested } => crate :: models :: DestEnumBasic ::
            Struct
            {
                field1 : field1, field2 : field2, nested : crate :: models ::
                DestStruct { a : nested.a, b : nested.b, s : nested.s, },
            },
        }, field : value.field,
    }
}
        }

        pub mod mod_1733505074333_br4h67khac9xkg {
            fn __map(value : crate :: models :: SourceStructWithEnum) -> crate :: models
:: DestStructWithEnum
{
    crate :: models :: DestStructWithEnum
    {
        enum_ : match value.enum_
        {
            crate :: models :: SourceEnumBasic :: Unit => crate :: models ::
            DestEnumBasic :: Unit, crate :: models :: SourceEnumBasic ::
            Touple(item_0, item_1) => crate :: models :: DestEnumBasic ::
            Touple(item_0, item_1), crate :: models :: SourceEnumBasic ::
            ToupleSingle(item_0) => crate :: models :: DestEnumBasic ::
            ToupleSingle(item_0), crate :: models :: SourceEnumBasic :: Struct
            { field1, field2, nested } => crate :: models :: DestEnumBasic ::
            Struct
            {
                field1 : field1, field2 : field2, nested : crate :: models ::
                DestStruct { a : nested.a, b : nested.b, s : nested.s, },
            },
        }, field : value.field,
    }
}
        }

        pub mod mod_1733505074392_rdrz5yd7ha8cd1 {
            fn map_proto_struct(value : crate :: protogen :: example :: Person) -> crate
:: protogen :: example :: HomoSepiens
{
    crate :: protogen :: example :: HomoSepiens
    {
        first_name : value.first_name, last_name : value.last_name, gender :
        value.gender.map(| v |
        {
            crate :: protogen :: example :: HomoSepiensGender
            {
                gender :
                v.gender.map(| v |
                {
                    match v
                    {
                        crate :: protogen :: example :: gender :: Gender ::
                        Male(item_0) => crate :: protogen :: example ::
                        homo_sepiens_gender :: Gender :: Male(item_0), crate ::
                        protogen :: example :: gender :: Gender :: Female(item_0) =>
                        crate :: protogen :: example :: homo_sepiens_gender ::
                        Gender :: Female(item_0),
                    }
                }),
            }
        }),
    }
}
        }

        pub mod mod_1733505074392_1aybgqphjbmppy {
            fn map_proto_struct(value : crate :: protogen :: example :: Person) -> crate
:: protogen :: example :: HomoSepiens
{
    crate :: protogen :: example :: HomoSepiens
    {
        first_name : value.first_name, last_name : value.last_name, gender :
        value.gender.map(| v |
        {
            crate :: protogen :: example :: HomoSepiensGender
            {
                gender :
                v.gender.map(| v |
                {
                    match v
                    {
                        crate :: protogen :: example :: gender :: Gender ::
                        Male(item_0) => crate :: protogen :: example ::
                        homo_sepiens_gender :: Gender :: Male(item_0), crate ::
                        protogen :: example :: gender :: Gender :: Female(item_0) =>
                        crate :: protogen :: example :: homo_sepiens_gender ::
                        Gender :: Female(item_0),
                    }
                }),
            }
        }),
    }
}
        }