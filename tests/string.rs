use cookie_rs::prelude::*;

#[test]
fn simple_string() {
    let expected = "name=value";
    let input = Cookie::new("name", "value");

    assert_eq!(input.to_string(), expected)
}

#[test]
fn empty_value() {
    let expected = "name=";
    let input = Cookie::new("name", "");

    assert_eq!(input.to_string(), expected)
}
