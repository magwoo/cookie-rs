pub use crate::cookie::Cookie;
pub use crate::cookie::CookieBuilder;
pub use crate::jar::CookieJar;

pub(crate) use prison::StringPrison;

mod cookie;
mod jar;
mod prison;

pub mod prelude {
    pub use crate::cookie::SameSite;
    pub use crate::*;
}
