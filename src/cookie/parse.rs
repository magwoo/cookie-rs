use std::borrow::Cow;
use std::time::Duration;

pub use self::error::*;
use super::Cookie;
use super::SameSite;

pub mod error;

impl<'a> Cookie<'a> {
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        let full_string: Cow<'a, str> = value.into();

        // SAFE `full_string` won't be released sooner
        let str = unsafe {
            let bytes = std::slice::from_raw_parts(full_string.as_ptr(), full_string.len());
            std::str::from_utf8_unchecked(bytes)
        };

        let mut cookie = parse_cookie(str)?;

        cookie.full_string = Some(full_string);

        Ok(cookie)
    }
}

fn parse_cookie(str: &str) -> Result<Cookie<'_>, ParseError> {
    let mut attributes = str.split(';');

    let (name, value) = attributes
        .next()
        .expect("Missing any attributes")
        .trim()
        .split_once('=')
        .ok_or(MissingPair::NameValue)?;

    if name.is_empty() {
        return Err(ParseError::EmptyName);
    }

    let mut cookie = Cookie::new(name, value);

    for attribute in attributes {
        let mut pair = attribute.trim().splitn(2, '=');

        let (name, value) = (pair.next().unwrap(), pair.next());

        match (name, value) {
            ("Domain", domain) => cookie.set_domain(domain.ok_or(MissingPair::Domain)?),
            ("Expires", expires) => cookie.set_expires(expires.ok_or(MissingPair::Expires)?),
            ("HttpOnly", _) => cookie.set_http_only(true),
            ("MaxAge", max_age) => cookie.set_max_age(Duration::from_secs(
                max_age.ok_or(MissingPair::MaxAge)?.parse()?,
            )),
            ("Partitioned", _) => cookie.set_partitioned(true),
            ("Path", path) => cookie.set_path(path.ok_or(MissingPair::Path)?),
            ("Secure", _) => cookie.set_secure(true),
            ("SameSite", same_site) => {
                cookie.set_same_site(SameSite::parse(same_site.ok_or(MissingPair::SameSite)?)?)
            }
            (name, _) => return Err(ParseError::UnknownAttribute(name.to_string())),
        }
    }

    Ok(cookie)
}

impl SameSite {
    pub fn parse(value: &str) -> Result<Self, ParseSameSiteError> {
        if value.eq_ignore_ascii_case("strict") {
            Ok(Self::Strict)
        } else if value.eq_ignore_ascii_case("lax") {
            Ok(Self::Lax)
        } else if value.eq_ignore_ascii_case("none") {
            Ok(Self::None)
        } else {
            Err(ParseSameSiteError::UnknownValue)
        }
    }
}
