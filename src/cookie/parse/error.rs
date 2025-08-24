use std::error::Error;
use std::fmt;
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
    UnknownValue(String),
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

impl Error for ParseError {}

impl Error for ParseSameSiteError {}

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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyName => write!(f, "the provided name is empty."),
            ParseError::MissingPair(pair) => write!(f, "missed pair: {pair}"),
            ParseError::UnknownAttribute(attr) => write!(f, "unknown attribute: {attr}"),
            ParseError::ParseMaxAgeError(err) => write!(f, "failed to parse Max-Age: {err}"),
            ParseError::ParseSameSiteError(err) => write!(f, "failed to parse SameSite: {err}"),
        }
    }
}

impl fmt::Display for ParseSameSiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseSameSiteError::UnknownValue(value) => {
                write!(f, "unknown SameSite value: {value}")
            }
        }
    }
}

impl fmt::Display for MissingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pair_name = match self {
            MissingPair::NameValue => "Name-Value",
            MissingPair::Domain => "Domain",
            MissingPair::Expires => "Expires",
            MissingPair::MaxAge => "Max-Age",
            MissingPair::Path => "Path",
            MissingPair::SameSite => "SameSite",
        };
        write!(f, "{pair_name}")
    }
}
