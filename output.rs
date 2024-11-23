fn __map(value : crate :: v2 :: models :: SourceStructWithEnum) -> crate :: v2
:: models :: DestStructWithEnum
{
    crate :: v2 :: models :: DestStructWithEnum
    { enum_ : match value.enum_ { _ => todo! (), }, field : value.field, }
}