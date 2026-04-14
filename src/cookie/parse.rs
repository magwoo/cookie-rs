//! Parsing utilities for `Cookie`.
//!
//! This module provides functionality for parsing `Cookie` instances from strings.
//! It supports both strict and lenient parsing modes and handles attributes such as
//! `Domain`, `Path`, `Max-Age`, `Secure`, and more.
//!
//! # Features
//! - Flexible parsing with `parse` and `parse_strict` methods.
//! - Detailed error handling using `ParseError` and `ParseSameSiteError`.
//! - Support for common cookie attributes.
//!
//! # Example
//! ```
//! use cookie_rs::prelude::*;
//!
//! let cookie = Cookie::parse("session=abc123; Path=/; Secure").unwrap();
//! assert_eq!(cookie.name(), "session");
//! assert_eq!(cookie.value(), "abc123");
//! assert_eq!(cookie.path(), Some("/"));
//! assert_eq!(cookie.secure(), Some(true));
//! ```
use std::borrow::Cow;
use std::time::Duration;

use super::Cookie;
use super::SameSite;
use crate::StringPrison;

pub use self::error::*;

pub mod error;

impl<'a> Cookie<'a> {
    /// Parses a cookie from a string in a lenient mode.
    ///
    /// In lenient mode, unknown attributes are ignored.
    ///
    /// # Arguments
    /// - `value`: The string representation of the cookie.
    ///
    /// # Returns
    /// A `Result` containing the parsed `Cookie` or a `ParseError`.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::parse("session=abc123; Secure").unwrap();
    /// assert_eq!(cookie.name(), "session");
    /// assert_eq!(cookie.value(), "abc123");
    /// assert_eq!(cookie.secure(), Some(true));
    /// ```
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        Self::inner_parse(value.into(), false)
    }

    /// Parses a cookie from a string in a strict mode.
    ///
    /// In strict mode, unknown attributes cause an error.
    ///
    /// # Arguments
    /// - `value`: The string representation of the cookie.
    ///
    /// # Returns
    /// A `Result` containing the parsed `Cookie` or a `ParseError`.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let result = Cookie::parse_strict("session=abc123; UnknownAttr");
    /// assert!(result.is_err());
    /// ```
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
            max_age if name.eq_ignore_ascii_case("Max-Age") => {
                let secs: i64 = max_age.ok_or(MissingPair::MaxAge)?.parse()?;
                cookie.set_max_age(Duration::from_secs(secs.max(0) as u64))
            }
            _ if name.eq_ignore_ascii_case("Partitioned") => cookie.set_partitioned(true),
            path if name.eq_ignore_ascii_case("Path") => {
                cookie.set_path(path.ok_or(MissingPair::Path)?)
            }
            _ if name.eq_ignore_ascii_case("Secure") => cookie.set_secure(true),
            same_site if name.eq_ignore_ascii_case("SameSite") => {
                cookie.set_same_site(same_site.ok_or(MissingPair::SameSite)?.parse()?)
            }
            _ if strict => return Err(ParseError::UnknownAttribute(name.to_owned())),
            _ => continue,
        }
    }

    Ok(cookie)
}

impl std::str::FromStr for SameSite {
    type Err = ParseSameSiteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("strict") {
            Ok(Self::Strict)
        } else if s.eq_ignore_ascii_case("lax") {
            Ok(Self::Lax)
        } else if s.eq_ignore_ascii_case("none") {
            Ok(Self::None)
        } else {
            Err(ParseSameSiteError::UnknownValue(s.to_owned()))
        }
    }
}
