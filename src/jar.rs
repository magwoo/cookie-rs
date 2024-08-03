use std::collections::BTreeSet;

use crate::Cookie;

pub use self::changed::CookieChange;

mod changed;

#[derive(Debug, Clone)]
pub struct CookieJar<'a> {
    cookie: BTreeSet<Cookie<'a>>,
    changes: BTreeSet<CookieChange<'a>>,
}

impl<'a> CookieJar<'a> {
    pub fn get(&self, name: &str) -> Option<&Cookie<'a>> {
        self.cookie.get(name)
    }

    pub fn add(&mut self, cookie: Cookie<'a>) {
        self.changes.replace(CookieChange::create(cookie));
    }

    pub fn remove<C: Into<Cookie<'a>>>(&mut self, cookie: C) {
        self.changes.replace(CookieChange::delete(cookie.into()));
    }

    pub fn cookie(&self) -> &BTreeSet<Cookie<'a>> {
        &self.cookie
    }

    pub fn changes(&self) -> &BTreeSet<CookieChange<'a>> {
        &self.changes
    }
}
