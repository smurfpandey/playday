use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::{AuthUrl, ClientId, ClientSecret, StandardRevocableToken, TokenUrl};
use serde::Deserialize;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub oauth: BasicClient,
    pub tera: Tera,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct Auth0User {
    pub name: String,
    pub email: String,
}

pub type OAuthClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
>;
