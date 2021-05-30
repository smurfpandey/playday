use actix_web::{web, HttpRequest, HttpResponse, Responder};
use http::header::{HeaderValue, ACCEPT, AUTHORIZATION};
use http::method::Method;
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, AccessToken, CsrfToken, RedirectUrl, Scope, TokenResponse};
use std::borrow::Cow;
use std::env;
use url::Url;

use crate::types;

pub const MIME_TYPE_JSON: &str = "application/json";

pub async fn get_games() -> impl Responder {
    format!("hello from get games")
}

pub async fn login(req: HttpRequest, oauth: web::Data<types::OAuthClient>) -> impl Responder {
    let redirect_url = req
        .url_for("login_callback", &[""])
        .expect("Redirect URL not found");

    let (auth_url, _csrf_token) = &oauth
        .authorize_url(CsrfToken::new_random)
        .set_redirect_uri(Cow::Owned(
            RedirectUrl::new(redirect_url.to_string()).unwrap(),
        ))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    HttpResponse::TemporaryRedirect()
        .header("location", auth_url.to_string())
        .finish()
}

fn get_user(access_token: &AccessToken) -> types::Auth0User {
    let auth0_base_url = env::var("AUTH0_BASE_URL").expect("AUTH0_BASE_URL must be set");
    let req_url = Url::parse(format!("{}/userinfo", auth0_base_url).as_str()).unwrap();

    let userinfo_request = oauth2::HttpRequest {
        url: req_url,
        method: Method::GET,
        headers: vec![
            (ACCEPT, HeaderValue::from_static(MIME_TYPE_JSON)),
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("{} {}", "Bearer", access_token.secret()))
                    .expect("Invalid access token"),
            ),
        ]
        .into_iter()
        .collect(),
        body: Vec::new(),
    };

    let resp = http_client(userinfo_request).expect("Request failed");
    serde_json::from_slice(&resp.body).unwrap()
}

pub async fn login_callback(
    req: HttpRequest,
    oauth: web::Data<types::OAuthClient>,
    params: web::Query<types::AuthRequest>,
) -> impl Responder {
    let code = AuthorizationCode::new(params.code.clone());
    let _state = CsrfToken::new(params.state.clone());
    let redirect_url = req
        .url_for("login_callback", &[""])
        .expect("Redirect URL not found");

    // Exchange the code with a token.
    let token = &oauth
        .exchange_code(code)
        .set_redirect_uri(Cow::Owned(
            RedirectUrl::new(redirect_url.to_string()).unwrap(),
        ))
        .request(http_client)
        .expect("exchange_code failed");

    let auth0_user = get_user(token.access_token());

    println!("Hello {} with {}", auth0_user.name, auth0_user.email);

    HttpResponse::Ok().body("Callback")
}
