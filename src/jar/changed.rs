use crate::Cookie;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeStatus {
    Create,
    Delete,
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

    pub fn delete(cookie: Cookie<'a>) -> Self {
        Self {
            cookie,
            status: ChangeStatus::Delete,
        }
    }

    pub fn cookie(&self) -> &Cookie<'a> {
        &self.cookie
    }

    pub fn status(&self) -> ChangeStatus {
        self.status
    }

    pub fn into_cookie(self) -> Cookie<'a> {
        self.cookie
    }
}

impl<'a> PartialEq for CookieChange<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cookie == other.cookie && self.status == other.status
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
