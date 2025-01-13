#![doc = include_str!("../README.md")]

pub use crate::cookie::Cookie;
pub use crate::cookie::CookieBuilder;
pub use crate::jar::CookieJar;

pub(crate) use prison::StringPrison;

mod prison;

pub mod cookie;
pub mod jar;

pub mod error {
    pub use crate::cookie::parse::error::*;
}

pub mod prelude {
    pub use crate::cookie::Cookie;
    pub use crate::cookie::CookieBuilder;
    pub use crate::cookie::SameSite;
    pub use crate::jar::CookieJar;
}
