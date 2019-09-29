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
    Authentication(Authentication),
}

impl From<Authentication> for Error {
    fn from(e: Authentication) -> Self {
        Self::Authentication(e)
    }
}

type e = reqwest::header::ToStrError;

#[derive(Debug)]
pub enum Authentication {
    LoginError(ReqErr),
    ToStringError(reqwest::header::ToStrError),
    MissingHeaders,
    MissingCookie,
    SliceError,
}

from! {ReqErr, Authentication::LoginError}
from! {reqwest::header::ToStrError, Authentication::ToStringError}

#[derive(Debug)]
pub enum Application {
    SendError(ReqErr),
    SerdeJson(serde_json::Error),
}

from! {ReqErr, Application::SendError}
from! {serde_json::Error, Application::SerdeJson}

#[derive(Debug)]
pub enum Log {
    SendError(ReqErr),
    SerdeJson(serde_json::Error),
}

from! {ReqErr, Log::SendError}
from! {serde_json::Error, Log::SerdeJson}


#[derive(Debug)]
pub enum SyncError {
    SendError(ReqErr),

}

from! {ReqErr, SyncError::SendError}

#[derive(Debug)]
pub enum Transfer {}


#[derive(Debug)]
pub enum TorrentManagement {
    
}