use lazy_to_map_derive::lazy_map;
use std::path::PathBuf;

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

fn it_works() {
    let test = Test {
        left: 1,
        right: 2,
        expected: 3,
        name: "test1".to_string(),
    };

    lazy_map! {
        fn test_to_test_2(Test, Test2);
    };
}
