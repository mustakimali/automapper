fn __map(value : v2 :: SourceStruct2) -> v2 :: DestStruct2
{
    v2 :: DestStruct2
    {
        s : value.s, nested : v2 :: DestStruct
        { a : value.nested.a, b : value.nested.b, s : value.nested.s, },
    }
}