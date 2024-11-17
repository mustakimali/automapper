#![allow(unused)]

use automapper::{AutoMapsFrom, AutoMapsTo};

#[derive(Debug, Clone)]
struct SourceStruct {
    a: i32,
    b: u32,
    s: String,
}

#[derive(Debug, Clone)]
struct DestStruct {
    a: i32,
    b: u32,
    s: String,
}

#[test]
fn basic_struct() {
    use crate::v2;
    let input = SourceStruct {
        a: 1,
        b: 2,
        s: "hello".to_string(),
    };

    //TODO: support for `create::*` syntax
    automapper::map!(v2::SourceStruct, v2::DestStruct);
    let output = input.clone().map_to();

    assert_eq!(input.a, output.a);
    assert_eq!(input.b, output.b);
    assert_eq!(input.s, output.s);
}

#[derive(Debug, Clone)]
struct SourceStruct2 {
    s: String,
    nested: SourceStruct,
}

#[derive(Debug, Clone)]
struct DestStruct2 {
    s: String,
    nested: DestStruct,
}
#[derive(Debug, Clone)]
struct DestStruct3 {
    s: String,
    nested: DestStruct,
}

#[test]
fn nested_struct() {
    use crate::v2;
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
    automapper::map!(v2::SourceStruct2, v2::DestStruct2);
    automapper::map!(v2::SourceStruct2, v2::DestStruct3);

    let output: v2::DestStruct2 = input.clone().map_to();
    let output_2 = v2::DestStruct2::map_from(input.clone());

    assert_eq!(input.s, output.s);
    assert_eq!(input.nested.a, output.nested.a);
    assert_eq!(input.nested.b, output.nested.b);
    assert_eq!(input.nested.s, output.nested.s);
}
