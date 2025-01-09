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

    pub fn builder<N, V>(name: N, value: V) -> CookieBuilder<'a>
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        CookieBuilder::new(name, value)
    }

    pub fn set_domain<V: Into<Cow<'a, str>>>(&mut self, domain: V) {
        self.domain = Some(domain.into())
    }

    pub fn set_expires<V: Into<Cow<'a, str>>>(&mut self, expires: V) {
        self.expires = Some(expires.into());
    }

    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = Some(http_only);
    }

    pub fn set_max_age<V: Into<Duration>>(&mut self, max_age: V) {
        self.max_age = Some(max_age.into());
    }

    pub fn set_partitioned(&mut self, partitioned: bool) {
        self.partitioned = Some(partitioned);
    }

    pub fn set_path<V: Into<Cow<'a, str>>>(&mut self, path: V) {
        self.path = Some(path.into());
    }

    pub fn set_same_site(&mut self, same_site: SameSite) {
        self.same_site = Some(same_site);
    }

    pub fn set_secure(&mut self, secure: bool) {
        self.secure = Some(secure);
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }

    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    pub fn expires(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    pub fn http_only(&self) -> Option<bool> {
        self.http_only
    }

    pub fn max_age(&self) -> Option<Duration> {
        self.max_age
    }

    pub fn partitioned(&self) -> Option<bool> {
        self.partitioned
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn same_site(&self) -> Option<SameSite> {
        self.same_site
    }

    pub fn secure(&self) -> Option<bool> {
        self.secure
    }
}

impl<'a> PartialEq for Cookie<'a> {
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

impl<'a> Eq for Cookie<'a> {}

impl<'a> PartialOrd for Cookie<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Cookie<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl<'a> Borrow<str> for Cookie<'a> {
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

impl<'a> std::str::FromStr for Cookie<'a> {
    type Err = parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cookie::parse(s.to_owned())
    }
}

impl<'a> fmt::Display for Cookie<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)?;

        if let Some(domain) = self.domain.as_ref() {
            write!(f, "; Domain={}", domain)?;
        }

        if let Some(expires) = self.expires.as_ref() {
            write!(f, "; Expires={}", expires)?;
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
            write!(f, "; Path={}", path)?;
        }

        if let Some(same_site) = self.same_site {
            write!(f, "; SameSite={}", same_site)?;
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

impl<'a> Default for Cookie<'a> {
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
