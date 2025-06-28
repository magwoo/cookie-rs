use std::borrow::Cow;

use crate::Cookie;

#[derive(Debug, Clone)]
pub enum CookieChange<'a> {
    Create(Cookie<'a>),
    Remove(Cow<'a, str>),
}

impl<'a> CookieChange<'a> {
    pub fn create(cookie: Cookie<'a>) -> Self {
        Self::Create(cookie)
    }

    pub fn remove(name: Cow<'a, str>) -> Self {
        Self::Remove(name)
    }

    pub fn cookie(&self) -> Option<&Cookie<'a>> {
        match self {
            Self::Create(cookie) => Some(cookie),
            Self::Remove(_) => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Create(cookie) => cookie.name(),
            Self::Remove(name) => name.as_ref(),
        }
    }

    pub fn is_create(&self) -> bool {
        match self {
            Self::Create(_) => true,
            Self::Remove(_) => false,
        }
    }

    pub fn is_remove(&self) -> bool {
        !self.is_create()
    }

    pub fn into_cookie(self) -> Option<Cookie<'a>> {
        match self {
            Self::Create(cookie) => Some(cookie),
            Self::Remove(_) => None,
        }
    }

    pub fn as_header_value(&self) -> String {
        match self {
            Self::Create(cookie) => cookie.to_string(),
            Self::Remove(name) => format!("{}=removed; Max-Age=0", name),
        }
    }
}

impl PartialEq for CookieChange<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for CookieChange<'_> {}

impl PartialOrd for CookieChange<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CookieChange<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name().cmp(other.name())
    }
}
