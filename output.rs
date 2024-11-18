fn __map(value : crate :: v2 :: models :: SourceStruct3) -> crate :: v2 ::
models :: DestStruct4
{
    crate :: v2 :: models :: DestStruct4
    {
        s : value.s, nested : crate :: v2 :: models :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
        optional :
        value.optional.map(| v |
        {
            crate :: v2 :: models :: DestStruct { a : v.a, b : v.b, s : v.s, }
        }),
    }
}