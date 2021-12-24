use thiserror::Error;

use oauth2::{url::ParseError, reqwest::Error as OAuthReqwestError, RequestTokenError, basic::BasicErrorResponse};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

#[derive(Error, Debug)]
pub enum PhabOAuthError {
    #[error("Could not parse url")]
    UrlParseError(#[from] ParseError),
    #[error("Error during performing the oauth request")]
    OAuthReqwestError(#[from] OAuthReqwestError<ReqwestError>),
    #[error("Error during performing the request")]
    ReqwestError(#[from] ReqwestError),
    #[error("Error during performing the token request")]
    RequestTokenError(#[from] RequestTokenError<OAuthReqwestError<ReqwestError>, BasicErrorResponse>),
    #[error("Could not serialize/deserialize JSON")]
    SerdeJsonError(#[from] SerdeJsonError),
}