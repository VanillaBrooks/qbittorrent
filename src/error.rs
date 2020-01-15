use reqwest;
use serde_json;

type ReqErr = reqwest::Error;

macro_rules! from {
    ($root:path, $destination_enum:ident :: $path_:ident) => {
        impl From<$root> for $destination_enum {
            fn from(e: $root) -> Self {
                $destination_enum::$path_(e)
            }
        }
    };
}

#[derive(Debug)]
pub enum Error {
    ReqErr(ReqErr),
    ToStringError(reqwest::header::ToStrError),
    SerdeJson(serde_json::Error),
    SerdeUrl(serde_urlencoded::ser::Error),
    MissingHeaders,
    MissingCookie,
    SliceError,
}

from! {reqwest::header::ToStrError, Error::ToStringError}
from! {serde_urlencoded::ser::Error, Error::SerdeUrl}
from! {serde_json::Error, Error::SerdeJson}
from! {reqwest::Error, Error::ReqErr}
