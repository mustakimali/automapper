#![allow(dead_code)]
#![allow(unused_imports)]

use lazy_to_map_derive::lazy_map;
use nested::nested_inner::NestedDestInnerType;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
struct Test {
    left: u64,
    right: u64,
    expected: u64,
    name: String,
}

struct Test2 {
    left: u64,
    right: u64,
    expected: u64,
    name: String,
}

struct TestNestedField {
    left: u64,
    right: u64,
    expected: u64,
    name: Test,
    from_another_crate: PathBuf,
}

struct TestNestedFieldTo {
    left: u64,
    right: u64,
    expected: u64,
    from_another_crate: std::path::PathBuf,
}

// --
//
#[derive(Clone, Debug, PartialEq)]
struct DestType {
    field: DestInnerType,
}

#[derive(Clone, Debug, PartialEq)]
struct DestInnerType {
    inner_field: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct SourceType {
    field: SourceInnerType,
}

#[derive(Clone, Debug, PartialEq)]
struct SourceInnerType {
    inner_field: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct SourceInnerTypeWthDifferentInnerTypeCanBeCasted {
    inner_field: i32,
}

mod nested {
    #[derive(Clone, Debug, PartialEq)]
    pub struct NestedDestType {
        pub field: nested_inner::NestedDestInnerType,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct NestedSourceType {
        pub field: nested_inner::NestedSourceInnerType,
    }

    pub mod nested_inner {
        #[derive(Clone, Debug, PartialEq)]
        pub struct NestedDestInnerType {
            pub inner_field: u64,
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct NestedSourceInnerType {
            pub inner_field: u64,
        }
    }
}

#[test]
fn simple_field_to_field_mappping() {
    let input = Test {
        left: 1,
        right: 2,
        expected: 3,
        name: "test1".to_string(),
    };

    lazy_map! {
        fn test_to_test_2(Test, Test2);
    };
    let result = test_to_test_2(input.clone());
    assert_eq!(result.left, input.left);
    assert_eq!(result.right, input.right);
    assert_eq!(result.expected, input.expected);
    assert_eq!(result.name, input.name);
}

#[test]
fn mapping_nested_similar_types() {
    let input = SourceType {
        field: SourceInnerType { inner_field: 1 },
    };
    lazy_map! {
        fn source_to_dest(SourceType, DestType);
    };
    let result = source_to_dest(input.clone());
    assert_eq!(result.field.inner_field, input.field.inner_field);
}

#[test]
fn mapping_nested_similar_types_on_nested_mod() {
    use nested::{nested_inner::NestedSourceInnerType, NestedDestType, NestedSourceType};
    let input = NestedSourceType {
        field: NestedSourceInnerType { inner_field: 1 },
    };

    lazy_map! {
        fn source_to_dest(nested::NestedSourceType, nested::NestedDestType);
    };
    let result = source_to_dest(input.clone());
    assert_eq!(result.field.inner_field, input.field.inner_field);
}

#[test]
fn mapping_casts_primitive_types() {
    let input = SourceInnerType { inner_field: 50 };
    lazy_map! {
        fn source_to_dest(SourceInnerType, SourceInnerTypeWthDifferentInnerTypeCanBeCasted);
    };
    let result = source_to_dest(input.clone());

    assert_eq!(result.inner_field, input.inner_field as i32);
}

#[derive(Clone, Debug, PartialEq)]
struct SourceStructWithEnumField {
    enum_field: SourceEnum,
}
#[derive(Clone, Debug, PartialEq)]
struct DestStructWithEnumField {
    enum_field: DestEnum,
}
#[derive(Clone, Debug, PartialEq)]
enum SourceEnum {
    Variant1,
    Variant2,
}
#[derive(Clone, Debug, PartialEq)]
enum DestEnum {
    Variant1,
    Variant2,
}

#[test]
fn basic_enum_mapping() {
    let input = SourceStructWithEnumField {
        enum_field: SourceEnum::Variant1,
    };
    lazy_map! {
        fn source_to_dest(SourceStructWithEnumField, DestStructWithEnumField);
    };
    let result = source_to_dest(input.clone());
    assert_eq!(result.enum_field, DestEnum::Variant1);
}
