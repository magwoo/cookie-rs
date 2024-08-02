use crate::Cookie;

#[derive(Debug)]
pub(crate) enum ChangeStatus {
    Create,
    Delete,
}

#[derive(Debug)]
pub(crate) struct ChangedCookie<'a> {
    cookie: Cookie<'a>,
    status: ChangeStatus,
}

impl<'a> ChangedCookie<'a> {
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
}

impl<'a> PartialEq for ChangedCookie<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cookie.eq(&other.cookie)
    }
}

impl<'a> Eq for ChangedCookie<'a> {}

impl<'a> PartialOrd for ChangedCookie<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ChangedCookie<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cookie.cmp(&other.cookie)
    }
}
