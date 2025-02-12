use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::cookie::parse::ParseError;
use crate::{Cookie, StringPrison};

pub use self::changed::CookieChange;

mod changed;
mod parse;

/// A container for managing HTTP cookies.
///
/// `CookieJar` provides a way to store, retrieve, and manipulate cookies,
/// including tracking changes (additions and removals) and converting them
/// into HTTP headers.
#[derive(Debug, Clone, Default)]
pub struct CookieJar<'a> {
    prison: Option<StringPrison<'a>>,
    cookie: BTreeSet<Cookie<'a>>,
    changes: BTreeSet<CookieChange<'a>>,
}

impl<'a> CookieJar<'a> {
    /// Creates a new `CookieJar` with the given set of cookies.
    ///
    /// # Arguments
    /// - `cookie`: A collection of cookies to initialize the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    /// use std::collections::BTreeSet;
    ///
    /// let cookies = vec![Cookie::new("session", "abc123")].into_iter().collect::<BTreeSet<_>>();
    /// let jar = CookieJar::new(cookies);
    /// ```
    pub fn new<C: Into<BTreeSet<Cookie<'a>>>>(cookie: C) -> Self {
        Self {
            cookie: cookie.into(),
            ..Default::default()
        }
    }

    /// Retrieves a cookie by its name.
    ///
    /// # Arguments
    /// - `name`: The name of the cookie to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the cookie if found, or `None` if not.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    ///
    /// let cookie = jar.get("session");
    /// assert!(cookie.is_some());
    /// ```
    pub fn get(&self, name: &str) -> Option<&Cookie<'a>> {
        self.changes
            .iter()
            .filter_map(|c| c.is_create().then_some(c.cookie()).flatten())
            .find(|c| c.name() == name)
            .or_else(|| self.cookie.iter().find(|c| c.name() == name))
    }

    /// Adds a new cookie to the jar or replaces an existing one with the same name.
    ///
    /// # Arguments
    /// - `cookie`: The cookie to add to the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    /// ```
    pub fn add<C: Into<Cookie<'a>>>(&mut self, cookie: C) {
        self.changes.replace(CookieChange::create(cookie.into()));
    }

    /// Adds a new cookie to the jar or replaces an existing one with the same name.
    ///
    /// # Arguments
    /// - `cookie`: The cookie to add to the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.set(Cookie::new("session", "abc123"));
    /// ```
    ///
    /// > alias for `CookieJar::add`
    pub fn set<C: Into<Cookie<'a>>>(&mut self, cookie: C) {
        self.add(cookie);
    }

    /// Removes a cookie from the jar by its name.
    ///
    /// # Arguments
    /// - `name`: The name of the cookie to remove.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    /// jar.remove("session");
    /// ```
    pub fn remove<N: Into<Cow<'a, str>>>(&mut self, name: N) {
        self.changes.replace(CookieChange::remove(name.into()));
    }

    /// Returns a reference to all cookies currently stored in the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    ///
    /// let cookies = jar.cookie();
    /// assert_eq!(cookies.len(), 1);
    /// ```
    pub fn cookie(&self) -> BTreeSet<&Cookie<'a>> {
        let mut cookie = BTreeSet::new();

        self.changes
            .iter()
            .filter_map(|c| c.cookie())
            .for_each(|c| {
                cookie.insert(c);
            });

        self.cookie.iter().for_each(|c| {
            cookie.insert(c);
        });

        cookie
    }

    /// Returns a reference to all changes (additions and removals) in the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    /// jar.remove("session");
    ///
    /// let changes = jar.changes();
    /// assert_eq!(changes.len(), 1);
    /// ```
    pub fn changes(&self) -> &BTreeSet<CookieChange<'a>> {
        &self.changes
    }

    /// Converts all changes in the jar to HTTP header values.
    ///
    /// # Returns
    /// A vector of strings representing `Set-Cookie` header values.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    ///
    /// let headers = jar.as_header_values();
    /// assert!(headers.iter().any(|h| h.starts_with("session=abc123")));
    /// ```
    pub fn as_header_values(&self) -> Vec<String> {
        self.changes.iter().map(|c| c.as_header_value()).collect()
    }
}

impl std::str::FromStr for CookieJar<'_> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s.to_owned())
    }
}
