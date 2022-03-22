use sae_library::*;

#[test]
fn string_test() {
    let mut str = String::from("");
    assert!(str.is_empty_or_white_space());
    str.push_str("Hello world!");
    assert!(!str.is_empty_or_white_space());
}
