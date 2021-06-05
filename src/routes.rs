use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use diesel::prelude::PgConnection;
use http::header::{HeaderValue, ACCEPT, AUTHORIZATION};
use http::method::Method;
use oauth2::reqwest::http_client;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, RedirectUrl, Scope, TokenResponse};
use std::borrow::Cow;
use std::env;
use tera::{Context, Tera};
use url::Url;
use uuid::Uuid;

use crate::db;
use crate::models;
use crate::types;

pub const MIME_TYPE_JSON: &str = "application/json";

pub async fn home(tera: web::Data<Tera>) -> impl Responder {
    let mut tera_data = Context::new();

    let games = [
        models::Game {
            id: Uuid::new_v4(),
            title: String::from("Horizon Forbidden West"),
            poster_url: String::from(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2gvu.jpg",
            ),
            publisher: String::from("Sony Interactive Entertainment"),
        },
        models::Game {
            id: Uuid::new_v4(),
            title: String::from("Halo Infinite"),
            poster_url: String::from(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2dto.jpg",
            ),
            publisher: String::from("Xbox Game Studios"),
        },
    ];
    tera_data.insert("title", "Playday");
    tera_data.insert("games", &games);

    let rendered = tera.render("index.html", &tera_data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn login(
    req: HttpRequest,
    oauth: web::Data<types::OAuthClient>,
    id: Identity,
) -> impl Responder {
    let redirect_url = req
        .url_for("login_callback", &[""])
        .expect("Redirect URL not found");

    if let Some(_id) = id.identity() {
        return HttpResponse::TemporaryRedirect()
            .header("location", "/")
            .finish();
    }

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

fn login_user(
    db_conn: &PgConnection,
    auth0_user: &types::Auth0User,
) -> Result<models::User, diesel::result::Error> {
    let user = db::get_user_by_email(db_conn, &auth0_user.email);

    match user {
        Ok(user) => {
            let now_utc = Utc::now().naive_utc();
            if let Some(app_user) = user {
                let _update = db::update_user_login_time(db_conn, app_user.id);
                return Ok(app_user);
            } else {
                let app_user = models::User {
                    id: Uuid::new_v4(),
                    name: auth0_user.name.to_owned(),
                    email: auth0_user.email.to_owned(),
                    created_at: now_utc,
                    last_login: now_utc,
                };
                let _user_saved = db::create_user(db_conn, &app_user);
                return Ok(app_user);
            }
        }
        Err(error) => {
            log::error!("Error getting user! {}", error);
            return Err(error);
        }
    };
}

pub async fn login_callback(
    req: HttpRequest,
    pool: web::Data<types::DBPool>,
    oauth: web::Data<types::OAuthClient>,
    params: web::Query<types::AuthRequest>,
    id: Identity,
) -> Result<HttpResponse, Error> {
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

    // log::info!("Hello {} with {}", auth0_user.name, user_email);

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || login_user(&conn, &auth0_user))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let user_string = serde_json::to_string(&user).unwrap();
    id.remember(user_string);
    log::info!("Logged in as {}!", &user.id);

    Ok(HttpResponse::TemporaryRedirect()
        .header("location", "/")
        .finish())
}
