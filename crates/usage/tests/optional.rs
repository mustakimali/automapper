use automapper::{AutoMapsFrom, AutoMapsTo};
use usage::models::{DestStruct, DestStruct4, SourceStruct, SourceStruct3};

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
    let expected_output = DestStruct4 {
        s: "hello".to_string(),
        nested: DestStruct {
            a: 1,
            b: 2,
            s: "world".to_string(),
        },
        optional: Some(DestStruct {
            a: 3,
            b: 4,
            s: "optional".to_string(),
        }),
    };

    //
    // mapping implemented in lib.rs
    //

    let output = input.clone().map_to();
    assert_eq!(output, expected_output);

    // another way (using trait AutoMapsFrom)
    let output = DestStruct4::map_from(input.clone());
    assert_eq!(output, expected_output);
}
