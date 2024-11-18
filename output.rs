fn __map(value : SourceStruct3) -> DestStruct4
{
    v2 :: models :: DestStruct4
    {
        s : value.s, nested : v2 :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        { v2 :: models :: DestStruct { a : v.a, b : v.b, s : v.s, } }),
    }
}