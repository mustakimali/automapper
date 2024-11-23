#![allow(unused)]

use crate::v2;
use automapper::{AutoMapsFrom, AutoMapsTo};
use models::*;

pub(crate) mod models;

mod tests {
    use automapper::{AutoMapsFrom, AutoMapsTo};

    #[test]
    fn basic_struct() {
        use crate::v2::models;
        let input = crate::v2::SourceStruct {
            a: 1,
            b: 2,
            s: "hello".to_string(),
        };

        automapper::map!(models::SourceStruct, models::DestStruct);
        let output = input.clone().map_to();

        assert_eq!(input.a, output.a);
        assert_eq!(input.b, output.b);
        assert_eq!(input.s, output.s);
    }
}

#[test]
fn nested_struct() {
    let input = SourceStruct2 {
        s: "hello".to_string(),
        nested: SourceStruct {
            a: 1,
            b: 2,
            s: "world".to_string(),
        },
    };

    // automapper::map! {
    //     fn mapping(v2::SourceStruct2 -> v2::DestStruct2);
    // };
    // let output = mapping(input.clone());
    automapper::map!(SourceStruct2, DestStruct2);
    automapper::map!(SourceStruct2, DestStruct3);

    let output: v2::DestStruct2 = input.clone().map_to();
    let output_2 = v2::DestStruct2::map_from(input.clone());

    assert_eq!(input.s, output.s);
    assert_eq!(input.nested.a, output.nested.a);
    assert_eq!(input.nested.b, output.nested.b);
    assert_eq!(input.nested.s, output.nested.s);
}

#[test]
fn optional_fields() {
    let input = SourceStruct3 {
        s: "hello".to_string(),
        nested: SourceStruct {
            a: 1,
            b: 2,
            s: "world".to_string(),
        },
        optional: Some(SourceStruct {
            a: 3,
            b: 4,
            s: "optional".to_string(),
        }),
    };

    automapper::map!(SourceStruct3, DestStruct4);
    let output = input.clone().map_to();

    assert_eq!(input.s, output.s);
    assert_eq!(input.nested.a, output.nested.a);
    assert_eq!(input.nested.b, output.nested.b);
    assert_eq!(input.nested.s, output.nested.s);

    assert!(output.optional.is_some());

    let optional = output.optional.unwrap();
    let expected_optional = input.optional.unwrap();
    assert_eq!(expected_optional.a, optional.a);
    assert_eq!(expected_optional.b, optional.b);
    assert_eq!(expected_optional.s, optional.s);
}

#[test]
fn simple_enum() {
    let input = SourceStructWithEnum {
        enum_: SourceEnumBasic::Struct {
            field1: 32,
            field2: "hello".to_string(),
            nested: SourceStruct {
                a: 1,
                b: 2,
                s: "world".to_string(),
            },
        },
        field: "yoyo".to_string(),
    };
    automapper::map!(SourceStructWithEnum, DestStructWithEnum);
}
