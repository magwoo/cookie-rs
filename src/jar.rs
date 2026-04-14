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
    /// Creates a new `CookieJar` with the given collection of cookies.
    ///
    /// # Arguments
    /// - `cookie`: An iterable of cookies to initialize the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let jar = CookieJar::new(vec![Cookie::new("session", "abc123")]);
    /// ```
    pub fn new(cookie: impl IntoIterator<Item = Cookie<'a>>) -> Self {
        Self {
            cookie: cookie.into_iter().collect(),
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
        match self.changes.iter().find(|c| c.name() == name) {
            Some(change) => change.cookie(),
            None => self.cookie.iter().find(|c| c.name() == name),
        }
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

    /// Returns an iterator over all cookies currently stored in the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    ///
    /// assert_eq!(jar.cookie().count(), 1);
    /// ```
    pub fn cookie(&self) -> impl Iterator<Item = &Cookie<'a>> + '_ {
        self.changes.iter().filter_map(|c| c.cookie()).chain(
            self.cookie
                .iter()
                .filter(|c| !self.changes.iter().any(|ch| ch.name() == c.name())),
        )
    }

    /// Returns the number of cookies in the jar.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut jar = CookieJar::default();
    /// jar.add(Cookie::new("session", "abc123"));
    ///
    /// assert_eq!(jar.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.cookie().count()
    }

    /// Returns `true` if the jar contains no cookies.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let jar = CookieJar::default();
    /// assert!(jar.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cookie().next().is_none()
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

    /// Converts all pending changes in the jar to `Set-Cookie` response header values.
    ///
    /// # Returns
    /// A vector of strings, each representing a `Set-Cookie` header value.
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
