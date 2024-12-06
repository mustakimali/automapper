

        pub mod mod_1733501794523_rxdksg5z3dj6q6 {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733501794528_mzjktx3m3ah7bp {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733501794579_xc0xfw8jtab7b3 {
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

        pub mod mod_1733501794587_z9k1b0kvxgrd68 {
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

        pub mod mod_1733501794638_7pdghez6mtj931 {
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

        pub mod mod_1733501794646_bbkkg74b8zaex2 {
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

        pub mod mod_1733501794693_861fxsyqv61fw4 {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733501794702_krnb9x7br61bby {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733501794750_1ypzczhg1q61zk {
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

        pub mod mod_1733501794760_435nw6pqj3gv07 {
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

        pub mod mod_1733501794806_xvf1dqk4q5tvtw {
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

        pub mod mod_1733501794818_9rcr1q7efz5vtc {
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