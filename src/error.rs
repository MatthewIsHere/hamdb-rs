use thiserror::Error;
use crate::parsing;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CallsignParsing(#[from] parsing::CallsignParseError),
    #[error("failed to send api request")]
    Http(#[source] reqwest::Error),
    #[error("failed to parse api response")]
    BodyParsing(#[source] reqwest::Error),
    #[error("request to api timed out")]
    Timeout(#[source] reqwest::Error),
    #[error("callsign `{0}` was not found")]
    NotFound(String)
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
