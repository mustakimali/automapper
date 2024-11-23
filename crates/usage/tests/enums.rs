use std::time::SystemTime;

use automapper::AutoMapsFrom;

use usage::models::{
    DestEnumBasic, DestStruct, DestStructWithEnum, SourceEnumBasic, SourceStruct,
    SourceStructWithEnum,
};

#[test]
fn enum_struct_variant() {
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
    let expected_output = DestStructWithEnum {
        enum_: DestEnumBasic::Struct {
            field1: 32,
            field2: "hello".to_string(),
            nested: DestStruct {
                a: 1,
                b: 2,
                s: "world".to_string(),
            },
        },
        field: "yoyo".to_string(),
    };

    // mapping implemented above:
    // automapper::map!(SourceStructWithEnum, DestStructWithEnum);
    let output = DestStructWithEnum::map_from(input.clone());

    assert_eq!(output, expected_output);
}

#[test]
fn enum_touple_variant() {
    let random_text = random_string();
    let input = SourceStructWithEnum {
        enum_: SourceEnumBasic::Touple(222, 2323),
        field: random_text.clone(),
    };
    let expected_output = DestStructWithEnum {
        enum_: DestEnumBasic::Touple(222, 2323),
        field: random_text,
    };

    // mapping implemented in lib.rs

    let output = DestStructWithEnum::map_from(input.clone());

    assert_eq!(output, expected_output);
}

#[test]
fn enum_unit_variant() {
    let random_text = random_string();
    let input = SourceStructWithEnum {
        enum_: SourceEnumBasic::Unit,
        field: random_text.clone(),
    };
    let expected_output = DestStructWithEnum {
        enum_: DestEnumBasic::Unit,
        field: random_text,
    };

    // mapping implemented in lib.rs

    let output = DestStructWithEnum::map_from(input.clone());

    assert_eq!(output, expected_output);
}

fn random_string() -> String {
    format!("{:?}", SystemTime::now())
}
