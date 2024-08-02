use std::borrow::Cow;
use std::time::Duration;

use super::{Cookie, SameSite};

pub struct CookieBuilder<'a>(Cookie<'a>);

impl<'a> CookieBuilder<'a> {
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Self(Cookie::new(name, value))
    }

    pub fn domain<V: Into<Cow<'a, str>>>(mut self, domain: V) -> Self {
        self.0.set_domain(domain);

        self
    }

    pub fn expires<V: Into<Cow<'a, str>>>(mut self, expires: V) -> Self {
        self.0.set_expires(expires);

        self
    }

    pub fn http_only(mut self, http_only: bool) -> Self {
        self.0.set_http_only(http_only);

        self
    }

    pub fn max_age<V: Into<Duration>>(mut self, max_age: V) -> Self {
        self.0.set_max_age(max_age);

        self
    }

    pub fn partitioned(mut self, partitioned: bool) -> Self {
        self.0.set_partitioned(partitioned);

        self
    }

    pub fn path<V: Into<Cow<'a, str>>>(mut self, path: V) -> Self {
        self.0.set_path(path);

        self
    }

    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.0.set_same_site(same_site);

        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.0.set_secure(secure);

        self
    }

    pub fn build(self) -> Cookie<'a> {
        self.0
    }
}
