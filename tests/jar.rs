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

#[test]
fn cookie_jar_get_after_remove_from_parsed() {
    let mut jar = CookieJar::parse("session=abc123; user=bob").unwrap();

    jar.remove("session");

    assert!(jar.get("session").is_none());
    assert!(jar.get("user").is_some());
}

#[test]
fn cookie_jar_cookie_set_excludes_removed_from_parsed() {
    let mut jar = CookieJar::parse("a=1; b=2; c=3").unwrap();

    jar.remove("b");

    let cookies = jar.cookie();
    assert_eq!(cookies.len(), 2);
    assert!(cookies.iter().any(|c| c.name() == "a"));
    assert!(!cookies.iter().any(|c| c.name() == "b"));
    assert!(cookies.iter().any(|c| c.name() == "c"));
}

#[test]
fn cookie_jar_remove_then_add_same_name() {
    let mut jar = CookieJar::parse("session=old").unwrap();

    jar.remove("session");
    jar.add(Cookie::new("session", "new"));

    assert_eq!(jar.get("session").unwrap().value(), "new");
}

#[test]
fn cookie_jar_remove_nonexistent() {
    let mut jar = CookieJar::default();

    jar.remove("ghost");

    assert!(jar.get("ghost").is_none());
    assert!(jar.cookie().is_empty());
}

#[test]
fn cookie_jar_changes_count_after_operations() {
    let mut jar = CookieJar::default();

    jar.add(Cookie::new("x", "1"));
    jar.remove("x");

    assert_eq!(jar.changes().len(), 1);
    assert!(jar.get("x").is_none());
}

#[test]
fn cookie_jar_get_unknown_returns_none() {
    let jar = CookieJar::parse("a=1; b=2").unwrap();

    assert!(jar.get("c").is_none());
}

#[test]
fn cookie_jar_cookie_count_after_add_and_remove() {
    let mut jar = CookieJar::default();

    jar.add(Cookie::new("a", "1"));
    jar.add(Cookie::new("b", "2"));
    jar.add(Cookie::new("c", "3"));
    jar.remove("b");

    assert_eq!(jar.cookie().len(), 2);
    assert!(jar.get("a").is_some());
    assert!(jar.get("b").is_none());
    assert!(jar.get("c").is_some());
}
