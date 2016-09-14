extern crate niancat;

use niancat::dictionary::{Dictionary, CheckWord};
use niancat::types::Word;

#[test]
fn init_dictionary_test() {
    let result = Dictionary::from_file("tests/test_dictionary.txt");

    match result {
        Err(ex) => assert!(false, "Failed to read dictionary from file because: {}", ex),
        Ok(d) => {
            assert!(d.is_solution(&Word("ABCDEFGHI".into())));
            assert!(d.is_solution(&Word("GALLTJUTA".into())));
            assert!(d.is_solution(&Word("UVWXYZÅÄÖ".into())));
            assert!(!d.is_solution(&Word("ABC".into())));
            assert!(!d.is_solution(&Word("ABCDEF".into())));
        }
    }
}