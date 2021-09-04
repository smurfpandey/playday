use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::StandardRevocableToken;
use serde::Deserialize;

use std::sync::Arc;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

pub type CeleryApp = Arc<celery::Celery<celery::broker::AMQPBroker>>;
