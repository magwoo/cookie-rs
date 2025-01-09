use cookie_rs::prelude::*;

#[test]
fn cookie_jar_add_cookie() {
    let mut jar = CookieJar::default();
    let cookie = Cookie::new("name", "value");

    jar.add(cookie.clone());

    assert_eq!(jar.get("name"), Some(&cookie));
}

#[test]
fn cookie_jar_remove_cookie() {
    let mut jar = CookieJar::default();
    let cookie = Cookie::new("name", "value");

    jar.add(cookie.clone());
    jar.remove(cookie.name());

    assert!(jar.get("name").is_none());
}

#[test]
fn cookie_jar_empty_string() {
    let cookie_count = CookieJar::parse("").map(|j| j.cookie().len());

    assert_eq!(cookie_count, Ok(0))
}

#[test]
fn cookie_jar_empty_string2() {
    let cookie_count = CookieJar::parse(" ").map(|j| j.cookie().len());

    assert_eq!(cookie_count, Ok(0))
}

#[test]
fn cookie_jar_empty_string3() {
    let cookie_count = CookieJar::parse(";").map(|j| j.cookie().len());

    assert_eq!(cookie_count, Ok(0))
}

#[test]
fn cookie_jar_add_multiple_cookies() {
    let mut jar = CookieJar::default();
    let cookie1 = Cookie::new("name1", "value1");
    let cookie2 = Cookie::new("name2", "value2");

    jar.add(cookie1.clone());
    jar.add(cookie2.clone());

    assert_eq!(jar.get("name1"), Some(&cookie1));
    assert_eq!(jar.get("name2"), Some(&cookie2));
}

#[test]
fn cookie_jar_to_header_values() {
    let mut jar = CookieJar::default();
    let cookie1 = Cookie::builder("name1", "value1")
        .path("/")
        .secure(true)
        .build();
    let cookie2 = Cookie::builder("name2", "value2").http_only(true).build();

    jar.add(cookie1);
    jar.add(cookie2);

    let headers = jar.as_header_values();

    assert!(headers.contains(&"name1=value1; Path=/; Secure".to_string()));
    assert!(headers.contains(&"name2=value2; HttpOnly".to_string()));
}

#[test]
fn cookie_jar_parse_cookie_header() {
    let input = "name1=value1; name2=value2";
    let jar = CookieJar::parse(input).unwrap();

    let cookie1 = jar.get("name1").unwrap();
    assert_eq!(cookie1.name(), "name1");
    assert_eq!(cookie1.value(), "value1");

    let cookie2 = jar.get("name2").unwrap();
    assert_eq!(cookie2.name(), "name2");
    assert_eq!(cookie2.value(), "value2");
}

#[test]
fn cookie_jar_empty() {
    let jar = CookieJar::default();
    assert!(jar.cookie().is_empty());
}

#[test]
fn cookie_jar_overwrite_cookie() {
    let mut jar = CookieJar::default();
    let cookie1 = Cookie::new("name", "value1");
    let cookie2 = Cookie::new("name", "value2");

    jar.add(cookie1.clone());
    jar.add(cookie2.clone());

    assert_eq!(jar.get("name"), Some(&cookie2));
}
