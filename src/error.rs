use thiserror::Error;

use oauth2::{url::ParseError, reqwest::{Error as OAuthReqwestError, HttpClientError}, RequestTokenError, StandardErrorResponse, basic::BasicErrorResponseType};
use serde_json::Error as SerdeJsonError;
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub enum PhabOAuthError {
    #[error("Could not parse url")]
    UrlParseError(#[from] ParseError),
    #[error("Error during performing the oauth request")]
    OAuthReqwestError(#[from] OAuthReqwestError<HttpClientError>),
    #[error("Error during performing the request")]
    ReqwestError(#[from] HttpClientError),
    #[error("Error during performing the token request")]
    RequestTokenError(#[from] RequestTokenError<HttpClientError, StandardErrorResponse<BasicErrorResponseType>>),
    #[error("Could not serialize/deserialize JSON")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("Could not read response body as utf-8 text")]
    FromUtf8Error(#[from] FromUtf8Error)
}