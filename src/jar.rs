use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::cookie::parse::ParseError;
use crate::{Cookie, StringPrison};

pub use self::changed::CookieChange;

mod changed;
mod parse;

#[derive(Debug, Clone, Default)]
pub struct CookieJar<'a> {
    prison: Option<StringPrison<'a>>,
    cookie: BTreeSet<Cookie<'a>>,
    changes: BTreeSet<CookieChange<'a>>,
}

impl<'a> CookieJar<'a> {
    pub fn new<C: Into<BTreeSet<Cookie<'a>>>>(cookie: C) -> Self {
        Self {
            cookie: cookie.into(),
            ..Default::default()
        }
    }

    pub fn get(&self, name: &str) -> Option<&Cookie<'a>> {
        self.changes
            .iter()
            .filter_map(|c| c.is_create().then_some(c.cookie()).flatten())
            .find(|c| c.name() == name)
            .or_else(|| self.cookie.iter().find(|c| c.name() == name))
    }

    pub fn add<C: Into<Cookie<'a>>>(&mut self, cookie: C) {
        self.changes.replace(CookieChange::create(cookie.into()));
    }

    pub fn remove<N: Into<Cow<'a, str>>>(&mut self, name: N) {
        self.changes.replace(CookieChange::remove(name.into()));
    }

    pub fn cookie(&self) -> &BTreeSet<Cookie<'a>> {
        &self.cookie
    }

    pub fn changes(&self) -> &BTreeSet<CookieChange<'a>> {
        &self.changes
    }

    pub fn as_header_values(&self) -> Vec<String> {
        self.changes.iter().map(|c| c.as_header_value()).collect()
    }
}

impl<'a> std::str::FromStr for CookieJar<'a> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s.to_owned())
    }
}
