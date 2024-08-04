use std::borrow::Cow;

use crate::cookie::parse::ParseError;
use crate::{Cookie, StringPrison};

use super::CookieJar;

impl<'a> CookieJar<'a> {
    pub fn parse<V: Into<Cow<'a, str>>>(value: V) -> Result<Self, ParseError> {
        let prison = StringPrison::new(value);

        let str = unsafe { prison.get() };

        let mut jar = parse_jar(str)?;
        jar.prison = Some(prison);

        Ok(jar)
    }
}

fn parse_jar(str: &str) -> Result<CookieJar<'_>, ParseError> {
    let mut jar = CookieJar::default();
    let cookie = str.split(';');

    for pair in cookie {
        jar.cookie.insert(Cookie::parse(pair.trim())?);
    }

    Ok(jar)
}
