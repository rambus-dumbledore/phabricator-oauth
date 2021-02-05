use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenInspectionResponse};
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, CsrfToken, AuthorizationCode, AccessToken};
use oauth2::reqwest::http_client;
use std::borrow::Cow;
use oauth2::url::Url;
use reqwest;

use serde::{Deserialize};
use serde_json;

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
    client: oauth2::Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenInspectionResponse>
}

impl PhabOAuthClient {
    pub fn new(phid: String, secret: String, redirect_url: String, phabricator_url: String) -> Result<PhabOAuthClient> {
        let client = BasicClient::new(
            ClientId::new(phid),
            Some(ClientSecret::new(secret)),
            AuthUrl::new(format!("{}/oauthserver/auth/", phabricator_url))?,
            Some(TokenUrl::new(format!("{}/oauthserver/token/", phabricator_url))?)
        )
        .set_redirect_url(RedirectUrl::new(redirect_url.clone())?);

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
            .set_redirect_url(Cow::Owned(url))
            .url();

        Ok(auth_url)
    }

    pub fn get_token(&self, code: String) -> Result<BasicTokenResponse> {
        let token_response = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request(http_client)?;

        Ok(token_response)
    }

    pub fn get_user(&self, token: &AccessToken) -> Option<PhabricatorUser> {
        let request_url = format!("{}/api/user.whoami?access_token={}", self.phabricator_url, token.secret());
        let response = reqwest::blocking::get(request_url.as_str()).unwrap().text().unwrap();
        let user_result: OAuthResponse<PhabricatorUser>= serde_json::from_str(response.as_str()).unwrap();
        user_result.result
    }
}