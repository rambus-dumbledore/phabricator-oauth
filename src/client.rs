use oauth2::{
    basic::{
        BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenResponse, BasicTokenType
    },
    http::{HeaderMap, Method},
    reqwest::async_http_client,
    url::Url,
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, HttpRequest, RedirectUrl,
    StandardRevocableToken, TokenUrl
};
use serde_json;
use std::borrow::Cow;
use serde::Deserialize;

use crate::error::PhabOAuthError;
use crate::user::PhabricatorUser;

type Result<T> = std::result::Result<T, PhabOAuthError>;

#[derive(Deserialize, Debug)]
struct OAuthResponse<T> {
    result: Option<T>,
    // error_code: Option<String>,
    // error_info: Option<String>
}

pub struct PhabOAuthClient {
    redirect_url: String,
    phabricator_url: String,
    client: oauth2::Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse>
}

impl PhabOAuthClient {
    pub fn new(phid: String, secret: String, redirect_url: String, phabricator_url: String) -> Result<PhabOAuthClient> {
        let client = BasicClient::new(
            ClientId::new(phid),
            Some(ClientSecret::new(secret)),
            AuthUrl::new(format!("{}/oauthserver/auth/", phabricator_url))?,
            Some(TokenUrl::new(format!("{}/oauthserver/token/", phabricator_url))?)
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.clone())?);

        Ok(
            PhabOAuthClient {
                redirect_url,
                phabricator_url,
                client
            }
        )
    }

    pub fn get_auth_url(&self) -> Result<(Url, CsrfToken)> {
        let url = RedirectUrl::new(self.redirect_url.clone())?;

        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .set_redirect_uri(Cow::Owned(url))
            .url();

        Ok((auth_url, csrf_token))
    }

    pub async fn get_token(&self, code: String) -> Result<BasicTokenResponse> {
        let token_response = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client).await?;

        Ok(token_response)
    }

    pub async fn get_user(&self, token: &AccessToken) -> Result<Option<PhabricatorUser>> {
        let request_url = format!("{}/api/user.whoami?access_token={}", self.phabricator_url, token.secret());
        let request = HttpRequest{
            url: Url::parse(request_url.as_str())?,
            headers: HeaderMap::new(),
            method: Method::GET,
            body: vec![]
        };
        let response = async_http_client(request).await?;
        let json = String::from_utf8(response.body)?;
        let user_result: OAuthResponse<PhabricatorUser> = serde_json::from_str(json.as_str())?;
        Ok(user_result.result)
    }
}