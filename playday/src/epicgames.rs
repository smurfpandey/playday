use std::borrow::Cow;
use std::env;

use anyhow::Result;
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    StandardTokenResponse, TokenUrl,
};
use oauth2::{RedirectUrl, Scope};
use url::Url;

use crate::types;

pub struct EpicGames {
    oauth_client: types::OAuthClient,
}

impl EpicGames {
    pub fn new() -> Result<EpicGames> {
        let client_id = ClientId::new(
            env::var("EPICGAMES_CLIENT_ID").expect("EPICGAMES_CLIENT_ID must be set"),
        );
        let client_secret = ClientSecret::new(
            env::var("EPICGAMES_CLIENT_SECRET").expect("EPICGAMES_CLIENT_SECRET must be set"),
        );
        let auth_url = AuthUrl::new("https://www.epicgames.com/id/authorize".to_string()).unwrap();
        let token_url =
            TokenUrl::new("https://api.epicgames.dev/epic/oauth/v1/token".to_string()).unwrap();

        let oauth_client = BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url),
        )
        .set_auth_type(AuthType::BasicAuthNC);

        Ok(EpicGames {
            oauth_client: oauth_client
        })
    }

    pub fn get_auth_url(&self, redirect_url: &str) -> (Url, CsrfToken) {
        self.oauth_client
            .authorize_url(CsrfToken::new_random)
            .set_redirect_uri(Cow::Owned(
                RedirectUrl::new(redirect_url.to_string()).unwrap(),
            ))
            .add_scope(Scope::new("basic_profile".to_string()))
            .url()
    }

    pub fn exchange_code_for_token(
        &self,
        auth_code: &str,
        redirect_url: &str,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let code = AuthorizationCode::new(auth_code.to_string());

        let token = self
            .oauth_client
            .exchange_code(code)
            .set_redirect_uri(Cow::Owned(
                RedirectUrl::new(redirect_url.to_string()).unwrap(),
            ))
            .request(http_client)
            .unwrap();

        Ok(token)
    }
}
