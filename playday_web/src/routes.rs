use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use diesel::prelude::PgConnection;
use http::header::{HeaderValue, ACCEPT, AUTHORIZATION};
use http::method::Method;
use oauth2::reqwest::http_client;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, RedirectUrl, Scope, TokenResponse};
use qstring::QString;
use std::borrow::Cow;
use std::env;
use tera::{Context, Tera};
use url::Url;
use uuid::Uuid;

use crate::db;
use crate::igdb::{IGDBGame, IGDB};
use crate::models;
use crate::types;

pub const MIME_TYPE_JSON: &str = "application/json";

fn is_logged_in(id: Identity) -> Option<models::User> {
    if let Some(user_info) = id.identity() {
        let user: models::User = serde_json::from_str(&user_info).unwrap();

        return Some(user);
    } else {
        return None;
    }
}

pub async fn home(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut tera_data = Context::new();

    if let Some(user) = is_logged_in(id) {
        tera_data.insert("user", &user);
        tera_data.insert("str_user", &serde_json::to_string(&user).unwrap());
    } else {
        return HttpResponse::TemporaryRedirect()
            .header("location", "/login")
            .finish();
    }

    tera_data.insert("title", "Playday");

    let rendered = tera.render("index.html", &tera_data).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(rendered)
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

pub async fn search_igdb_games(
    req: HttpRequest,
    id: Identity,
    igdb_client: web::Data<IGDB>,
) -> Result<HttpResponse, Error> {
    if let Some(_user) = is_logged_in(id) {
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let query_str = req.query_string(); // "name=ferret"
    let qs = QString::from(query_str);
    let keyword = qs.get("keyword"); // "ferret"

    if keyword.is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let search_keyword = keyword.unwrap();
    let games = igdb_client.search_games(search_keyword).unwrap();

    return Ok(HttpResponse::Ok().json(games));
}

pub async fn add_games_to_wishlist(
    pool: web::Data<types::DBPool>,
    id: Identity,
    str_igdb_game: String,
) -> Result<HttpResponse, Error> {
    match is_logged_in(id) {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(user) => {
            // use web::block to offload blocking Diesel code without blocking server thread
            Ok(web::block(move || {
                let igdb_games: Vec<IGDBGame> = serde_json::from_str(&str_igdb_game).unwrap();
                let mut wished_games: Vec<models::WishedGame> = Vec::new();

                for game in igdb_games.iter() {
                    wished_games.push(models::WishedGame {
                        id: Uuid::new_v4(),
                        user_id: user.id.to_owned(),
                        title: game.name.to_owned(),
                        igdb_id: game.id.to_owned(),
                        added_on: Utc::now().naive_utc(),
                        igdb_info: serde_json::to_value(game).unwrap(),
                        pc_release_date: game.get_pc_release_date()
                    });
                }

                let conn = pool.get().expect("couldn't get db connection from pool");
                let _added = match db::add_games_to_wishlist(&conn, &wished_games) {
                    Ok(added) => added,
                    Err(e) => return Err(e),
                };

                Ok(wished_games)
            })
            .await
            .map(|wished_games| HttpResponse::Ok().json(wished_games))
            .map_err(|error| {
                log::error!("Error saving games to wishlist! {}", error);
                HttpResponse::InternalServerError().finish()
            })?)
        }
    }
}

pub async fn get_games_in_wishlist(
    pool: web::Data<types::DBPool>,
    id: Identity,
) -> Result<HttpResponse, Error> {
    match is_logged_in(id) {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(user) => {
            // use web::block to offload blocking Diesel code without blocking server thread
            Ok(web::block(move || {
                let conn = pool.get().expect("couldn't get db connection from pool");
                let wished_games = match db::get_games_from_wishlist(&conn, user.id) {
                    Ok(games) => games,
                    Err(e) => return Err(e),
                };

                Ok(wished_games)
            })
            .await
            .map(|wished_games| HttpResponse::Ok().json(wished_games))
            .map_err(|error| {
                log::error!("Error getting games from wishlist! {}", error);
                HttpResponse::InternalServerError().finish()
            })?)
        }
    }
}

// DELETE /api/wishlist/{gameId}
pub async fn remove_game_from_wishlist(
    pool: web::Data<types::DBPool>,
    id: Identity,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match is_logged_in(id) {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(user) => {
            let game_id = path.into_inner();
            let conn = pool.get().expect("couldn't get db connection from pool");

            // use web::block to offload blocking Diesel code without blocking server thread
            let _remove =
                web::block(move || db::remove_game_from_wishlist(&conn, user.id, game_id))
                    .await
                    .map_err(|e| {
                        log::error!("Error removing game from wishlist! {}", e);
                        HttpResponse::InternalServerError().finish()
                    })?;

            Ok(HttpResponse::NoContent().finish())
        }
    }
}
