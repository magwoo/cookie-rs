use std::borrow::Cow;

use crate::cookie::parse::ParseError;
use crate::{Cookie, StringPrison};

use super::CookieJar;

impl<'a> CookieJar<'a> {
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        Self::inner_parse(value.into(), false)
    }

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
    let cookie = str.split(';');

    for pair in cookie {
        jar.cookie
            .insert(Cookie::inner_parse(pair.trim().into(), strict)?);
    }

    Ok(jar)
}
