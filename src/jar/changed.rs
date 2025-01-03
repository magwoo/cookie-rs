use crate::Cookie;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeStatus {
    Create,
    Remove,
}

#[derive(Debug, Clone)]
pub struct CookieChange<'a> {
    cookie: Cookie<'a>,
    status: ChangeStatus,
}

impl<'a> CookieChange<'a> {
    pub fn create(cookie: Cookie<'a>) -> Self {
        Self {
            cookie,
            status: ChangeStatus::Create,
        }
    }

    pub fn remove(cookie: Cookie<'a>) -> Self {
        Self {
            cookie,
            status: ChangeStatus::Remove,
        }
    }

    pub fn cookie(&self) -> &Cookie<'a> {
        &self.cookie
    }

    pub fn status(&self) -> ChangeStatus {
        self.status
    }

    pub fn is_create(&self) -> bool {
        self.status() == ChangeStatus::Create
    }

    pub fn is_remove(&self) -> bool {
        self.status() == ChangeStatus::Remove
    }

    pub fn into_cookie(self) -> Cookie<'a> {
        self.cookie
    }

    pub fn as_header_value(&self) -> String {
        match self.status {
            ChangeStatus::Create => self.cookie.to_string(),
            ChangeStatus::Remove => format!("{}=removed; Max-Age=0", self.cookie.name()),
        }
    }
}

impl<'a> PartialEq for CookieChange<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cookie.name() == other.cookie.name()
    }
}

impl<'a> Eq for CookieChange<'a> {}

impl<'a> PartialOrd for CookieChange<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for CookieChange<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cookie.cmp(&other.cookie)
    }
}
