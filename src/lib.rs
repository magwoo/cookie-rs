pub use crate::cookie::Cookie;
pub use crate::cookie::CookieBuilder;
pub use crate::jar::CookieJar;

pub(crate) use prison::StringPrison;

mod prison;

pub mod cookie;
pub mod jar;

pub mod prelude {
    pub use crate::cookie::SameSite;
    pub use crate::*;
}
