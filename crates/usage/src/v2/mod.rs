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
    let input = SourceStruct {
        a: 1,
        b: 2,
        s: "hello".to_string(),
    };
    automapper::map! {
        fn mapping(SourceStruct, DestStruct);
    };
    let output = mapping(input.clone());
}
