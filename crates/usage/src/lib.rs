#![allow(dead_code)]
#![allow(unused_imports)]
use lazy_to_map_derive::lazy_map;
use std::path::PathBuf;

#[derive(Clone)]
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
#[derive(Clone)]
struct DestType {
    field: DestInnerType,
}

#[derive(Clone)]
struct DestInnerType {
    inner_field: u64,
}

#[derive(Clone)]
struct SourceType {
    field: SourceInnerType,
}

#[derive(Clone)]
struct SourceInnerType {
    inner_field: u64,
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
