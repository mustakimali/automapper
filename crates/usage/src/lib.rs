use lazy_to_map_derive::lazy_map;

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

#[test]
fn rustdoc_json() {
    let json_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path("Cargo.toml")
        .document_private_items(true)
        .all_features(true)
        .build()
        .unwrap();

    panic!("Wrote rustdoc JSON to {:?}", &json_path);
}
