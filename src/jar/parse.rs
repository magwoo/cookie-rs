use std::borrow::Cow;

use crate::cookie::parse::ParseError;
use crate::{Cookie, StringPrison};

use super::CookieJar;

impl<'a> CookieJar<'a> {
    /// Parses a `Cookie` request header value into a `CookieJar`.
    ///
    /// Expects the `Cookie` header format: `name=value` pairs separated by `; `.
    /// In lenient mode, unknown attributes are ignored.
    ///
    /// # Arguments
    /// - `value`: The `Cookie` header string.
    ///
    /// # Returns
    /// A `Result` containing the parsed `CookieJar` or a `ParseError`.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let jar = CookieJar::parse("session=abc123; user=bob").unwrap();
    /// assert_eq!(jar.get("session").unwrap().value(), "abc123");
    /// ```
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        Self::inner_parse(value.into(), false)
    }

    /// Parses a `Cookie` request header value into a `CookieJar` in strict mode.
    ///
    /// Expects the `Cookie` header format: `name=value` pairs separated by `; `.
    /// In strict mode, unknown attributes cause an error.
    ///
    /// # Arguments
    /// - `value`: The `Cookie` header string.
    ///
    /// # Returns
    /// A `Result` containing the parsed `CookieJar` or a `ParseError`.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let jar = CookieJar::parse_strict("session=abc123; user=bob").unwrap();
    /// assert_eq!(jar.get("user").unwrap().value(), "bob");
    /// ```
    pub fn parse_strict<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        Self::inner_parse(value.into(), true)
    }

    fn inner_parse(value: Cow<'a, str>, strict: bool) -> Result<Self, ParseError> {
        let prison = StringPrison::new(value);

        // SAFETY: prison and slice owned by the same struct
        let str = unsafe { prison.get() };

        let mut jar = parse_jar(str, strict)?;
        jar.prison = Some(prison);

        Ok(jar)
    }
}

fn parse_jar(str: &str, strict: bool) -> Result<CookieJar<'_>, ParseError> {
    let mut jar = CookieJar::default();
    let cookie = str.split(';').map(|p| p.trim()).filter(|p| !p.is_empty());

    for pair in cookie {
        jar.cookie.insert(Cookie::inner_parse(pair.into(), strict)?);
    }

    Ok(jar)
}
