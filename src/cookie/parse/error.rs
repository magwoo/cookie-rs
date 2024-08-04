use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyName,
    MissingPair(MissingPair),
    UnknownAttribute(String),
    ParseMaxAgeError(ParseIntError),
    ParseSameSiteError(ParseSameSiteError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseSameSiteError {
    UnknownValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissingPair {
    NameValue,
    Domain,
    Expires,
    MaxAge,
    Path,
    SameSite,
}

impl From<MissingPair> for ParseError {
    fn from(value: MissingPair) -> Self {
        Self::MissingPair(value)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseMaxAgeError(value)
    }
}

impl From<ParseSameSiteError> for ParseError {
    fn from(value: ParseSameSiteError) -> Self {
        Self::ParseSameSiteError(value)
    }
}
