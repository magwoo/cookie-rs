use cookie::parse::ParseError;
use cookie_rs::prelude::*;

#[test]
fn simple_cookie() {
    let expected = Ok(Cookie::new("name", "value"));
    let input = "name=value";

    assert_eq!(Cookie::parse(input), expected)
}

#[test]
fn empty_name() {
    let expected = Err(ParseError::EmptyName);
    let input = "=value";

    assert_eq!(Cookie::parse(input), expected)
}

#[test]
fn empty_value() {
    let expected = Ok(Cookie::new("key", ""));
    let input = "key=";

    assert_eq!(Cookie::parse(input), expected)
}

#[test]
fn empty_input() {
    let expected = Err(ParseError::MissingPair(
        cookie::parse::MissingPair::NameValue,
    ));
    let input = "";

    assert_eq!(Cookie::parse(input), expected)
}
