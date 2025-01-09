use std::borrow::Cow;
use std::time::Duration;

use super::{Cookie, SameSite};

/// A builder for constructing `Cookie` instances with optional attributes.
///
/// `CookieBuilder` provides a convenient way to create a `Cookie` by chaining
/// methods to set attributes such as `Domain`, `Path`, `Secure`, and others.
pub struct CookieBuilder<'a>(Cookie<'a>);

impl<'a> CookieBuilder<'a> {
    /// Creates a new `CookieBuilder` with the specified name and value.
    ///
    /// # Arguments
    /// - `name`: The name of the cookie.
    /// - `value`: The value of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let builder = CookieBuilder::new("session", "abc123");
    /// ```
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Self(Cookie::new(name, value))
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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .domain("example.com")
    ///     .build();
    /// assert_eq!(cookie.domain(), Some("example.com"));
    /// ```
    pub fn domain<V: Into<Cow<'a, str>>>(mut self, domain: V) -> Self {
        self.0.set_domain(domain);

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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .expires("Wed, 21 Oct 2025 07:28:00 GMT")
    ///     .build();
    /// assert_eq!(cookie.expires(), Some("Wed, 21 Oct 2025 07:28:00 GMT"));
    /// ```
    pub fn expires<V: Into<Cow<'a, str>>>(mut self, expires: V) -> Self {
        self.0.set_expires(expires);

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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .http_only(true)
    ///     .build();
    /// assert_eq!(cookie.http_only(), Some(true));
    /// ```
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.0.set_http_only(http_only);

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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .max_age(Duration::from_secs(3600))
    ///     .build();
    /// assert_eq!(cookie.max_age(), Some(Duration::from_secs(3600)));
    /// ```
    pub fn max_age<V: Into<Duration>>(mut self, max_age: V) -> Self {
        self.0.set_max_age(max_age);

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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .partitioned(true)
    ///     .build();
    /// assert_eq!(cookie.partitioned(), Some(true));
    /// ```
    pub fn partitioned(mut self, partitioned: bool) -> Self {
        self.0.set_partitioned(partitioned);

        self
    }

    /// Sets the path for the cookie.
    ///
    /// # Arguments
    /// - `path`: The path attribute of the cookie.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .path("/")
    ///     .build();
    /// assert_eq!(cookie.path(), Some("/"));
    /// ```
    pub fn path<V: Into<Cow<'a, str>>>(mut self, path: V) -> Self {
        self.0.set_path(path);

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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .same_site(SameSite::Lax)
    ///     .build();
    /// assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    /// ```
    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.0.set_same_site(same_site);

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
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .secure(true)
    ///     .build();
    /// assert_eq!(cookie.secure(), Some(true));
    /// ```
    pub fn secure(mut self, secure: bool) -> Self {
        self.0.set_secure(secure);

        self
    }

    /// Finalizes the builder and returns the constructed `Cookie`.
    ///
    /// # Example
    /// ```
    /// use cookie_rs::prelude::*;
    ///
    /// let cookie = CookieBuilder::new("session", "abc123")
    ///     .secure(true)
    ///     .build();
    /// assert_eq!(cookie.name(), "session");
    /// assert_eq!(cookie.value(), "abc123");
    /// ```
    pub fn build(self) -> Cookie<'a> {
        self.0
    }
}
