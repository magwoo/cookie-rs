use std::collections::BTreeSet;

use self::changed::ChangedCookie;
use crate::Cookie;

mod changed;

pub struct CookieJar<'a> {
    cookie: BTreeSet<Cookie<'a>>,
    changes: BTreeSet<ChangedCookie<'a>>,
}

impl<'a> CookieJar<'a> {
    pub fn get(&self, name: &str) -> Option<&Cookie<'a>> {
        self.cookie.get(name)
    }

    pub fn add(&mut self, cookie: Cookie<'a>) {
        self.changes.replace(ChangedCookie::create(cookie));
    }

    pub fn remove<C: Into<Cookie<'a>>>(&mut self, cookie: C) {
        self.changes.replace(ChangedCookie::delete(cookie.into()));
    }
}
