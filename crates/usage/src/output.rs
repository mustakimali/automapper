

        pub mod mod_1733685908960_29hqe3m40ztngt {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733685908966_3hs02fw91xq08s {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733685909021_0bb2fvgzn4r3kb {
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

        pub mod mod_1733685909025_ebze8v5cb9zq60 {
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

        pub mod mod_1733685909078_xm2qje69znj779 {
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

        pub mod mod_1733685909083_8rdg3rs89262sc {
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

        pub mod mod_1733685909132_npyh7k6n3j2xbc {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733685909141_7dwxk311qjyqsc {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733685909193_e8pbtrmrjjfpmc {
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

        pub mod mod_1733685909201_9dwdc3v6qz1fka {
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

        pub mod mod_1733685909248_v8eh7nx2h84n5g {
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

        pub mod mod_1733685909257_4a1xxpv569ptm6 {
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

        pub mod mod_1733685909305_e4r94fdby9rye5 {
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

        pub mod mod_1733685909315_7vhq42cqhhkaah {
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