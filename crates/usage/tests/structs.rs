#![allow(unused)]

use std::time::SystemTime;

use automapper::{AutoMapsFrom, AutoMapsTo};
use usage::models::{DestStruct2, SourceStruct, SourceStruct2};

mod tests {
    use automapper::{AutoMapsFrom, AutoMapsTo};
    use usage::models::SourceStruct;

    #[test]
    fn basic_struct() {
        let input = SourceStruct {
            a: 1,
            b: 2,
            s: "hello".to_string(),
        };

        // mapping implemented in lib.rs

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

    // mapping implemented in lib.rs

    let output: DestStruct2 = input.clone().map_to();
    let output_2 = DestStruct2::map_from(input.clone());

    assert_eq!(input.s, output.s);
    assert_eq!(input.nested.a, output.nested.a);
    assert_eq!(input.nested.b, output.nested.b);
    assert_eq!(input.nested.s, output.nested.s);
}
