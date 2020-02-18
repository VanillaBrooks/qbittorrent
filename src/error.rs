use reqwest;
use serde_json;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Request Error when talking to qbittorrent: {0}")]
    ReqErr(#[from] reqwest::Error),
    #[error("Could not convert reqwest header to string: {0}")]
    ToStringError(#[from] reqwest::header::ToStrError),
    #[error("Serde json could not correctly deserialize: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Serde_urlencoded could not serialize the url: {0}")]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),
    #[error("Header value was malformed: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Header value was not correctly set")]
    MissingHeaders,
    #[error("Cookie value was not correctly set")]
    MissingCookie,
    #[error("SLICE ERROR ??")]
    SliceError,
}
