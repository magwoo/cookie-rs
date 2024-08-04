use std::borrow::Cow;
use std::time::Duration;

pub use self::error::*;
use super::Cookie;
use super::SameSite;

pub mod error;

impl<'a> Cookie<'a> {
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        let full_string: Cow<'a, str> = value.into();

        // SAFETY: `full_string` won't be released sooner
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

        match value {
            domain if name.eq_ignore_ascii_case("Domain") => {
                cookie.set_domain(domain.ok_or(MissingPair::Domain)?)
            }
            expires if name.eq_ignore_ascii_case("Expires") => {
                cookie.set_expires(expires.ok_or(MissingPair::Expires)?)
            }
            _ if name.eq_ignore_ascii_case("HttpOnly") => cookie.set_http_only(true),
            max_age if name.eq_ignore_ascii_case("Max-Age") => cookie.set_max_age(
                Duration::from_secs(max_age.ok_or(MissingPair::MaxAge)?.parse()?),
            ),
            _ if name.eq_ignore_ascii_case("Partitioned") => cookie.set_partitioned(true),
            path if name.eq_ignore_ascii_case("Path") => {
                cookie.set_path(path.ok_or(MissingPair::Path)?)
            }
            _ if name.eq_ignore_ascii_case("Secure") => cookie.set_secure(true),
            same_site if name.eq_ignore_ascii_case("SameSite") => {
                cookie.set_same_site(SameSite::parse(same_site.ok_or(MissingPair::SameSite)?)?)
            }
            _ => return Err(ParseError::UnknownAttribute(name.to_string())),
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
