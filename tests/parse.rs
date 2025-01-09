use cookie::parse::ParseError;
use cookie_rs::prelude::*;

#[test]
fn simple_cookie() {
    let expected = Cookie::new("name", "value");
    let input = "name=value";

    assert_eq!(Cookie::parse(input), Ok(expected))
}

#[test]
fn empty_name() {
    let expected = Err(ParseError::EmptyName);
    let input = "=value";

    assert_eq!(Cookie::parse(input), expected)
}

#[test]
fn empty_value() {
    let expected = Cookie::new("key", "");
    let input = "key=";

    assert_eq!(Cookie::parse(input), Ok(expected))
}

#[test]
fn empty_input() {
    let expected = Err(ParseError::MissingPair(
        cookie::parse::MissingPair::NameValue,
    ));
    let input = "";

    assert_eq!(Cookie::parse(input), expected)
}

#[test]
fn cookie_with_domain() {
    let expected = Cookie::builder("name", "value")
        .domain("example.com")
        .build();
    let input = "name=value; Domain=example.com";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_path() {
    let expected = Cookie::builder("name", "value")
        .path("/path/to/resource")
        .build();
    let input = "name=value; Path=/path/to/resource";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_secure() {
    let expected = Cookie::builder("name", "value").secure(true).build();
    let input = "name=value; Secure";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_httponly() {
    let expected = Cookie::builder("name", "value").http_only(true).build();
    let input = "name=value; HttpOnly";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_max_age() {
    let expected = Cookie::builder("name", "value")
        .max_age(std::time::Duration::from_secs(3600))
        .build();
    let input = "name=value; Max-Age=3600";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_samesite_strict() {
    let expected = Cookie::builder("name", "value")
        .same_site(SameSite::Strict)
        .build();
    let input = "name=value; SameSite=Strict";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_samesite_lax() {
    let expected = Cookie::builder("name", "value")
        .same_site(SameSite::Lax)
        .build();
    let input = "name=value; SameSite=Lax";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_unknown_attribute_strict() {
    let expected = Err(ParseError::UnknownAttribute("UnknownAttr".to_string()));
    let input = "name=value; UnknownAttr";

    assert_eq!(Cookie::parse_strict(input), expected);
}

#[test]
fn cookie_with_unknown_attribute_non_strict() {
    let expected = Cookie::builder("name", "value").build();
    let input = "name=value; UnknownAttr";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_multiple_attributes() {
    let expected = Cookie::builder("name", "value")
        .domain("example.com")
        .path("/path")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .build();
    let input = "name=value; Domain=example.com; Path=/path; Secure; HttpOnly; SameSite=None";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn malformed_cookie_missing_equals() {
    let expected = Err(ParseError::MissingPair(
        cookie::parse::MissingPair::NameValue,
    ));
    let input = "namevalue";

    assert_eq!(Cookie::parse(input), expected);
}

#[test]
fn cookie_with_expires() {
    let expected = Cookie::builder("name", "value")
        .expires("Wed, 21 Oct 2025 07:28:00 GMT")
        .build();
    let input = "name=value; Expires=Wed, 21 Oct 2025 07:28:00 GMT";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_partitioned() {
    let expected = Cookie::builder("name", "value").partitioned(true).build();
    let input = "name=value; Partitioned";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_empty_attribute_value() {
    let expected = Cookie::builder("name", "value").path("").build();
    let input = "name=value; Path=";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_case_insensitive_attributes() {
    let expected = Cookie::builder("name", "value")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    let input = "name=value; secure; httponly; samesite=Lax";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_unexpected_whitespace() {
    let expected = Cookie::builder("name", "value")
        .domain("example.com")
        .path("/")
        .secure(true)
        .http_only(true)
        .build();
    let input = " name = value ; Domain = example.com ; Path = / ; Secure ; HttpOnly ";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_empty_pair() {
    let expected = Err(ParseError::MissingPair(
        cookie::parse::MissingPair::NameValue,
    ));
    let input = ";";

    assert_eq!(Cookie::parse(input), expected);
}

#[test]
fn cookie_with_duplicate_attributes() {
    let expected = Cookie::builder("name", "value").path("/second").build();
    let input = "name=value; Path=/first; Path=/second";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_invalid_max_age() {
    let expected = Err(ParseError::ParseMaxAgeError(
        "invalid".parse::<u64>().unwrap_err(),
    ));
    let input = "name=value; Max-Age=invalid";

    assert_eq!(Cookie::parse(input), expected);
}

#[test]
fn cookie_with_invalid_samesite_value() {
    let expected = Err(ParseError::ParseSameSiteError(
        cookie::parse::ParseSameSiteError::UnknownValue("InvalidValue".to_string()),
    ));
    let input = "name=value; SameSite=InvalidValue";

    assert_eq!(Cookie::parse(input), expected);
}

#[test]
fn cookie_with_trailing_semicolon() {
    let expected = Cookie::builder("name", "value").build();
    let input = "name=value;";

    assert_eq!(Cookie::parse(input), Ok(expected));
}

#[test]
fn cookie_with_invalid_format() {
    let expected = Err(ParseError::MissingPair(
        cookie::parse::MissingPair::NameValue,
    ));
    let input = "name-value";

    assert_eq!(Cookie::parse(input), expected);
}

#[test]
fn cookie_with_non_ascii_name() {
    let expected = Cookie::builder("имя", "значение").build();
    let input = "имя=значение";

    assert_eq!(Cookie::parse(input), Ok(expected));
}
