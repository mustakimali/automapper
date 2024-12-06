

        pub mod mod_1733502366610_b3m5d55w2vwbz7 {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733502366619_2dt4qghjjg9wzy {
            fn __map(value : crate :: models :: SourceStruct) -> crate :: models ::
DestStruct
{ crate :: models :: DestStruct { a : value.a, b : value.b, s : value.s, } }
        }

        pub mod mod_1733502366681_dm57kqk9ammser {
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

        pub mod mod_1733502366683_gcrcgkp86nn7k0 {
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

        pub mod mod_1733502366751_x7y0930x1ks8rq {
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

        pub mod mod_1733502366752_6qbe8yawcvy8fk {
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

        pub mod mod_1733502366815_z30xb99p5kdhf3 {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733502366817_5f65skmnfw0761 {
            fn __map(value : crate :: models :: SourcePrim) -> crate :: models :: DestPrim
{ crate :: models :: DestPrim { a : value.a, } }
        }

        pub mod mod_1733502366881_m539j50m5spm7q {
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

        pub mod mod_1733502366897_0m3e84kv71swrj {
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

        pub mod mod_1733502366944_teaq4athhmsy86 {
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

        pub mod mod_1733502366961_b3ykjrkhb17kn1 {
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