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

#[test]
fn cookie_with_domain() {
    let expected = "name=value; Domain=example.com";
    let input = Cookie::builder("name", "value")
        .domain("example.com")
        .build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_path() {
    let expected = "name=value; Path=/path/to/resource";
    let input = Cookie::builder("name", "value")
        .path("/path/to/resource")
        .build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_secure() {
    let expected = "name=value; Secure";
    let input = Cookie::builder("name", "value").secure(true).build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_httponly() {
    let expected = "name=value; HttpOnly";
    let input = Cookie::builder("name", "value").http_only(true).build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_max_age() {
    let expected = "name=value; Max-Age=3600";
    let input = Cookie::builder("name", "value")
        .max_age(std::time::Duration::from_secs(3600))
        .build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_samesite_strict() {
    let expected = "name=value; SameSite=Strict";
    let input = Cookie::builder("name", "value")
        .same_site(SameSite::Strict)
        .build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_all_attributes() {
    let expected =
        "name=value; Domain=example.com; HttpOnly; Max-Age=3600; Path=/path; SameSite=Lax; Secure";
    let input = Cookie::builder("name", "value")
        .domain("example.com")
        .path("/path")
        .max_age(std::time::Duration::from_secs(3600))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_empty_name() {
    let expected = "=value";
    let input = Cookie::new("", "value");

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_empty_name_and_value() {
    let expected = "=";
    let input = Cookie::new("", "");

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_empty_attributes() {
    let expected = "name=value";
    let input = Cookie::builder("name", "value").build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_partitioned() {
    let expected = "name=value; Partitioned";
    let input = Cookie::builder("name", "value").partitioned(true).build();

    assert_eq!(input.to_string(), expected);
}

#[test]
fn cookie_with_expires() {
    let expected = "name=value; Expires=Wed, 21 Oct 2025 07:28:00 GMT";
    let input = Cookie::builder("name", "value")
        .expires("Wed, 21 Oct 2025 07:28:00 GMT")
        .build();

    assert_eq!(input.to_string(), expected);
}
