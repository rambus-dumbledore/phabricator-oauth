use oauth2::{
    ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, CsrfToken, AuthorizationCode, AccessToken, StandardRevocableToken,
    basic::{BasicClient, BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, BasicRevocationErrorResponse},
    reqwest::async_http_client,
    url::Url,
};
use reqwest;
use serde_json;
use std::borrow::Cow;
use serde::{Deserialize};

use crate::user::PhabricatorUser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
struct OAuthResponse<T> {
    result: Option<T>,
    error_code: Option<String>,
    error_info: Option<String>
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

    pub fn get_auth_url(&self) -> Result<Url> {
        let url = RedirectUrl::new(self.redirect_url.clone())?;

        let (auth_url, _csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .set_redirect_uri(Cow::Owned(url))
            .url();

        Ok(auth_url)
    }

    pub async fn get_token(&self, code: String) -> Result<BasicTokenResponse> {
        let token_response = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client).await?;

        Ok(token_response)
    }

    pub async fn get_user(&self, token: &AccessToken) -> Option<PhabricatorUser> {
        let request_url = format!("{}/api/user.whoami?access_token={}", self.phabricator_url, token.secret());
        let response = reqwest::get(request_url.as_str()).await.unwrap().text().await.unwrap();
        let user_result: OAuthResponse<PhabricatorUser>= serde_json::from_str(response.as_str()).unwrap();
        user_result.result
    }
}