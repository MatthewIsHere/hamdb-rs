//! Error types returned by the HamDB client.

use crate::parsing;
use thiserror::Error;

#[derive(Debug, Error)]
/// Top-level error returned by the crate.
pub enum Error {
    #[error(transparent)]
    /// Invalid user input while parsing the callsign.
    CallsignParsing(#[from] parsing::CallsignParseError),
    #[error("failed to send api request")]
    /// Network failure when issuing the HTTP request.
    Http(#[source] reqwest::Error),
    #[error("failed to parse api response")]
    /// The HamDB API returned a response body that could not be decoded.
    BodyParsing(#[source] reqwest::Error),
    #[error("request to api timed out")]
    /// The request exceeded the default timeout.
    Timeout(#[source] reqwest::Error),
    #[error("callsign `{0}` was not found")]
    /// The API reported the callsign as missing.
    NotFound(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_timeout() {
            Self::Timeout(value)
        } else if value.is_decode() {
            Self::BodyParsing(value)
        } else {
            Self::Http(value)
        }
    }
}
