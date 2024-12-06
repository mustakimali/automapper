use automapper::AutoMapsFrom;
use usage::{
    gen::random_string,
    models::{
        DestStruct, DestStruct2, DestStructWithResult, SourceStruct, SourceStruct2,
        SourceStructWithResult,
    },
};

#[test]
fn result_mapping() {
    let random_1 = random_string();
    let random_2 = random_string();
    let input = SourceStructWithResult {
        field: Ok(SourceStruct2 {
            s: random_1.clone(),
            nested: SourceStruct {
                a: 1,
                b: 2,
                s: random_2.clone(),
            },
        }),
    };
    let expected = DestStructWithResult {
        field: Ok(DestStruct2 {
            s: random_1,
            nested: DestStruct {
                a: 1,
                b: 2,
                s: random_2,
            },
        }),
    };

    let output = DestStructWithResult::map_from(input.clone());
    assert_eq!(output, expected);
}
