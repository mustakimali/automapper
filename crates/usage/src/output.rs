

        pub mod mod_1733261967465_3k6y15ewn8hesf {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733261967470_fejvq3xvvt3mvh {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733261967546_r6c9kcfar662cg {
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

        pub mod mod_1733261967555_1cckrmcyjy9ew3 {
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

        pub mod mod_1733261967643_z2jxmgspqvc1nn {
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

        pub mod mod_1733261967657_jhwnk2w8cjkp48 {
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

        pub mod mod_1733261967757_rp7zzdmmx3ymrd {
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

        pub mod mod_1733261967765_ekkj3b8a4b5eac {
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

        pub mod mod_1733261967846_dqz004qx14rjg4 {
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

        pub mod mod_1733261967859_7gw3zazaytfbma {
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