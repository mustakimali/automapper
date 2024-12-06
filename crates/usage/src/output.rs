

        pub mod mod_1733500713021_3vh9gyce3jr886 {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733500713031_nxs83fampj7jk9 {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733500713076_pd8b1haazh86vm {
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

        pub mod mod_1733500713087_q4jk00j7655fzr {
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

        pub mod mod_1733500713134_g0ezqg2c875pzh {
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

        pub mod mod_1733500713144_crgdxzxwmd3req {
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

        pub mod mod_1733500713192_61axwk4tsvfznh {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733500713201_87f9c11gwx14v0 {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733500713247_tc2af5nhpt25ff {
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

        pub mod mod_1733500713257_e74n38n42smbra {
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

        pub mod mod_1733500713304_8bcsd1zaznz6fy {
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

        pub mod mod_1733500713314_psg68f992w7fv4 {
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