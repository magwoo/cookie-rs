use std::borrow::Cow;
use std::time::Duration;

use super::Cookie;
use super::SameSite;
use crate::StringPrison;

pub use self::error::*;

pub mod error;

impl<'a> Cookie<'a> {
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        Self::inner_parse(value.into(), false)
    }

    pub fn parse_strict<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        Self::inner_parse(value.into(), true)
    }

    pub(crate) fn inner_parse(value: Cow<'a, str>, strict: bool) -> Result<Self, ParseError> {
        let prison = StringPrison::new(value);

        // SAFETY: prison and slice owned by the same struct
        let str = unsafe { prison.get() };

        let mut cookie = parse_cookie(str, strict)?;
        cookie.prison = Some(prison);

        Ok(cookie)
    }
}

fn parse_cookie(str: &str, strict: bool) -> Result<Cookie<'_>, ParseError> {
    let mut attributes = str.split(';');

    let (name, value) = attributes
        .next()
        .expect("Missing any attributes")
        .split_once('=')
        .ok_or(MissingPair::NameValue)?;

    let (name, value) = (name.trim(), value.trim());

    if name.is_empty() {
        return Err(ParseError::EmptyName);
    }

    let mut cookie = Cookie::new(name, value);

    for attribute in attributes {
        let mut pair = attribute.splitn(2, '=');

        let (name, value) = (
            pair.next().expect("missing any attribute name").trim(),
            pair.next().map(|v| v.trim()),
        );

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
            _ if strict => return Err(ParseError::UnknownAttribute(name.to_owned())),
            _ => continue,
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
            Err(ParseSameSiteError::UnknownValue(value.to_owned()))
        }
    }
}
