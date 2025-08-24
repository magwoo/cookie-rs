use std::borrow::{Borrow, Cow};
use std::fmt;
use std::time::Duration;

pub use self::builder::CookieBuilder;
use crate::StringPrison;

pub mod builder;
pub mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

/// Represents an HTTP cookie, including attributes such as domain, path, and expiration.
#[derive(Debug, Clone)]
pub struct Cookie<'a> {
    prison: Option<StringPrison<'a>>,
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    domain: Option<Cow<'a, str>>,
    expires: Option<Cow<'a, str>>,
    http_only: Option<bool>,
    max_age: Option<Duration>,
    partitioned: Option<bool>,
    path: Option<Cow<'a, str>>,
    same_site: Option<SameSite>,
    secure: Option<bool>,
}

impl<'a> Cookie<'a> {
    /// Creates a new `Cookie` with the specified name and value.
    ///
    /// # Arguments
    /// - `name`: The name of the cookie.
    /// - `value`: The value of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123");
    /// assert_eq!(cookie.name(), "session");
    /// assert_eq!(cookie.value(), "abc123");
    /// ```
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            value: value.into(),
            ..Default::default()
        }
    }

    /// Creates a `CookieBuilder` for constructing a `Cookie` with additional attributes.
    ///
    /// # Arguments
    /// - `name`: The name of the cookie.
    /// - `value`: The value of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::builder("session", "abc123")
    ///     .domain("example.com")
    ///     .secure(true)
    ///     .build();
    /// ```
    pub fn builder<N, V>(name: N, value: V) -> CookieBuilder<'a>
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        CookieBuilder::new(name, value)
    }

    /// Sets the domain for the cookie.
    ///
    /// # Arguments
    /// - `domain`: The domain attribute of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_domain("example.com");
    /// assert_eq!(cookie.domain(), Some("example.com"));
    /// ```
    pub fn set_domain<V: Into<Cow<'a, str>>>(&mut self, domain: V) {
        self.domain = Some(domain.into())
    }

    /// Sets the expiration date for the cookie.
    ///
    /// # Arguments
    /// - `expires`: The expiration date of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_expires("Wed, 21 Oct 2025 07:28:00 GMT");
    /// assert_eq!(cookie.expires(), Some("Wed, 21 Oct 2025 07:28:00 GMT"));
    /// ```
    pub fn set_expires<V: Into<Cow<'a, str>>>(&mut self, expires: V) {
        self.expires = Some(expires.into());
    }

    /// Sets the `HttpOnly` attribute for the cookie.
    ///
    /// # Arguments
    /// - `http_only`: Whether the cookie is HttpOnly.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_http_only(true);
    /// assert_eq!(cookie.http_only(), Some(true));
    /// ```
    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = Some(http_only);
    }

    /// Sets the maximum age for the cookie.
    ///
    /// # Arguments
    /// - `max_age`: The maximum age of the cookie as a `Duration`.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_max_age(Duration::from_secs(3600));
    /// assert_eq!(cookie.max_age(), Some(Duration::from_secs(3600)));
    /// ```
    pub fn set_max_age<V: Into<Duration>>(&mut self, max_age: V) {
        self.max_age = Some(max_age.into());
    }

    /// Sets the partitioned attribute for the cookie.
    ///
    /// # Arguments
    /// - `partitioned`: Whether the cookie is partitioned.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_partitioned(true);
    /// assert_eq!(cookie.partitioned(), Some(true));
    /// ```
    pub fn set_partitioned(&mut self, partitioned: bool) {
        self.partitioned = Some(partitioned);
    }

    /// Sets the path attribute for the cookie.
    ///
    /// # Arguments
    /// - `path`: The path attribute of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_path("/");
    /// assert_eq!(cookie.path(), Some("/"));
    /// ```
    pub fn set_path<V: Into<Cow<'a, str>>>(&mut self, path: V) {
        self.path = Some(path.into());
    }

    /// Sets the `SameSite` attribute for the cookie.
    ///
    /// # Arguments
    /// - `same_site`: The `SameSite` attribute for the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_same_site(SameSite::Lax);
    /// assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    /// ```
    pub fn set_same_site(&mut self, same_site: SameSite) {
        self.same_site = Some(same_site);
    }

    /// Sets the `Secure` attribute for the cookie.
    ///
    /// # Arguments
    /// - `secure`: Whether the cookie is secure.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_secure(true);
    /// assert_eq!(cookie.secure(), Some(true));
    /// ```
    pub fn set_secure(&mut self, secure: bool) {
        self.secure = Some(secure);
    }

    /// Sets the domain for the cookie.
    ///
    /// # Arguments
    /// - `domain`: The domain attribute of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_domain("example.com");
    ///
    /// assert_eq!(cookie.domain(), Some("example.com"));
    /// ```
    pub fn with_domain<V: Into<Cow<'a, str>>>(mut self, domain: V) -> Self {
        self.set_domain(domain);

        self
    }

    /// Sets the expiration date for the cookie.
    ///
    /// # Arguments
    /// - `expires`: The expiration date of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_expires("Wed, 21 Oct 2025 07:28:00 GMT");
    ///
    /// assert_eq!(cookie.expires(), Some("Wed, 21 Oct 2025 07:28:00 GMT"));
    /// ```
    pub fn with_expires<V: Into<Cow<'a, str>>>(mut self, expires: V) -> Self {
        self.set_expires(expires);

        self
    }

    /// Sets the `HttpOnly` attribute for the cookie.
    ///
    /// # Arguments
    /// - `http_only`: Whether the cookie is HttpOnly.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_http_only(true);
    ///
    /// assert_eq!(cookie.http_only(), Some(true));
    /// ```
    pub fn with_http_only(mut self, http_only: bool) -> Self {
        self.set_http_only(http_only);

        self
    }

    /// Sets the maximum age for the cookie.
    ///
    /// # Arguments
    /// - `max_age`: The maximum age of the cookie as a `Duration`.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_max_age(Duration::from_secs(3600));
    ///
    /// assert_eq!(cookie.max_age(), Some(Duration::from_secs(3600)));
    /// ```
    pub fn with_max_age<V: Into<Duration>>(mut self, max_age: V) -> Self {
        self.set_max_age(max_age);

        self
    }

    /// Sets the partitioned attribute for the cookie.
    ///
    /// # Arguments
    /// - `partitioned`: Whether the cookie is partitioned.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_partitioned(true);
    ///
    /// assert_eq!(cookie.partitioned(), Some(true));
    /// ```
    pub fn with_partitioned(mut self, partitioned: bool) -> Self {
        self.set_partitioned(partitioned);

        self
    }

    /// Sets the `Secure` attribute for the cookie.
    ///
    /// # Arguments
    /// - `secure`: Whether the cookie is secure.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_secure(true);
    ///
    /// assert_eq!(cookie.secure(), Some(true));
    /// ```
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.set_secure(secure);

        self
    }

    /// Sets the path attribute for the cookie.
    ///
    /// # Arguments
    /// - `path`: The path attribute of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_path("/");
    ///
    /// assert_eq!(cookie.path(), Some("/"));
    /// ```
    pub fn with_path<V: Into<Cow<'a, str>>>(mut self, path: V) -> Self {
        self.set_path(path);

        self
    }

    /// Sets the `SameSite` attribute for the cookie.
    ///
    /// # Arguments
    /// - `same_site`: The `SameSite` attribute for the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123").with_same_site(SameSite::Lax);
    ///
    /// assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    /// ```
    pub fn with_same_site(mut self, same_site: SameSite) -> Self {
        self.set_same_site(same_site);

        self
    }

    /// Returns the name of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123");
    /// assert_eq!(cookie.name(), "session");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the value of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = Cookie::new("session", "abc123");
    /// assert_eq!(cookie.value(), "abc123");
    /// ```
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }

    /// Returns the domain of the cookie, if set.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_domain("example.com");
    /// assert_eq!(cookie.domain(), Some("example.com"));
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    /// Returns the expiration date of the cookie, if set.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_expires("Wed, 21 Oct 2025 07:28:00 GMT");
    /// assert_eq!(cookie.expires(), Some("Wed, 21 Oct 2025 07:28:00 GMT"));
    /// ```
    pub fn expires(&self) -> Option<&str> {
        self.expires.as_deref()
    }

    /// Returns whether the cookie has the `HttpOnly` attribute set.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_http_only(true);
    /// assert_eq!(cookie.http_only(), Some(true));
    /// ```
    pub fn http_only(&self) -> Option<bool> {
        self.http_only
    }

    /// Returns the maximum age of the cookie, if set.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_max_age(Duration::from_secs(3600));
    /// assert_eq!(cookie.max_age(), Some(Duration::from_secs(3600)));
    /// ```
    pub fn max_age(&self) -> Option<Duration> {
        self.max_age
    }

    /// Returns whether the cookie is partitioned.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_partitioned(true);
    /// assert_eq!(cookie.partitioned(), Some(true));
    /// ```
    pub fn partitioned(&self) -> Option<bool> {
        self.partitioned
    }

    /// Returns the path of the cookie, if set.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_path("/");
    /// assert_eq!(cookie.path(), Some("/"));
    /// ```
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Returns the `SameSite` attribute of the cookie, if set.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_same_site(SameSite::Lax);
    /// assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    /// ```
    pub fn same_site(&self) -> Option<SameSite> {
        self.same_site
    }

    /// Returns whether the cookie has the `Secure` attribute set.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let mut cookie = Cookie::new("session", "abc123");
    /// cookie.set_secure(true);
    /// assert_eq!(cookie.secure(), Some(true));
    /// ```
    pub fn secure(&self) -> Option<bool> {
        self.secure
    }
}

impl PartialEq for Cookie<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self.domain.as_ref(), other.domain.as_ref()) {
            (Some(a), Some(b)) if a.eq_ignore_ascii_case(b) => (),
            (None, None) => (),
            _ => return false,
        }

        match (self.path.as_ref(), other.path.as_ref()) {
            (Some(a), Some(b)) if a.eq_ignore_ascii_case(b) => (),
            (None, None) => (),
            _ => return false,
        }

        self.name == other.name
            && self.value == other.value
            && self.expires == other.expires
            && self.http_only == other.http_only
            && self.max_age == other.max_age
            && self.partitioned == other.partitioned
            && self.same_site == other.same_site
            && self.secure == other.secure
    }
}

impl Eq for Cookie<'_> {}

impl PartialOrd for Cookie<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cookie<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Borrow<str> for Cookie<'_> {
    fn borrow(&self) -> &str {
        self.name()
    }
}

impl<'a> From<&'a str> for Cookie<'a> {
    fn from(value: &'a str) -> Self {
        Cookie::new(value, "")
    }
}

impl<'a> From<(&'a str, &'a str)> for Cookie<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        Cookie::new(value.0, value.1)
    }
}

impl std::str::FromStr for Cookie<'_> {
    type Err = parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cookie::parse(s.to_owned())
    }
}

impl fmt::Display for Cookie<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)?;

        if let Some(domain) = self.domain.as_ref() {
            write!(f, "; Domain={domain}")?;
        }

        if let Some(expires) = self.expires.as_ref() {
            write!(f, "; Expires={expires}")?;
        }

        if self.http_only.is_some_and(|v| v) {
            write!(f, "; HttpOnly")?;
        }

        if let Some(max_age) = self.max_age.as_ref() {
            write!(f, "; Max-Age={}", max_age.as_secs())?;
        }

        if self.partitioned.is_some_and(|v| v) {
            write!(f, "; Partitioned")?;
        }

        if let Some(path) = self.path.as_ref() {
            write!(f, "; Path={path}")?;
        }

        if let Some(same_site) = self.same_site {
            write!(f, "; SameSite={same_site}")?;
        }

        if self.secure.is_some_and(|v| v) {
            write!(f, "; Secure")?;
        }

        Ok(())
    }
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
        }
    }
}

impl Default for Cookie<'_> {
    fn default() -> Self {
        Self {
            prison: None,
            name: Cow::Borrowed(""),
            value: Cow::Borrowed(""),
            domain: None,
            expires: None,
            http_only: None,
            max_age: None,
            partitioned: None,
            path: None,
            same_site: None,
            secure: None,
        }
    }
}
