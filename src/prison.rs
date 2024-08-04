use std::borrow::Cow;

#[derive(Debug, Clone)]
pub(crate) struct StringPrison<'a>(Cow<'a, str>);

impl<'a> StringPrison<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(string: S) -> Self {
        Self(string.into())
    }

    pub unsafe fn get<'b>(&'a self) -> &'b str {
        let bytes = std::slice::from_raw_parts(self.0.as_ptr(), self.0.len());
        std::str::from_utf8_unchecked(bytes)
    }
}
