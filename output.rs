fn __map(value : v2 :: SourceStruct3) -> v2 :: DestStruct4
{
    v2 :: DestStruct4
    {
        s : value.s, nested : v2 :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        { v2 :: DestStruct { a : v.a, b : v.b, s : v.s, } }),
    }
}