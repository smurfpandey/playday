#[macro_use]
extern crate diesel;
pub mod schema;

use std::env;

use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::prelude::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use tera::{Tera};

mod auth;
mod db;
mod models;
mod routes;
mod types;
mod igdb;

fn establish_connection() -> types::DBPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}


fn get_oauth_client() -> types::OAuthClient {
    let client_id =
        ClientId::new(env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set"));
    let client_secret = ClientSecret::new(
        env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set"),
    );
    let auth0_base_url = env::var("AUTH0_BASE_URL").expect("AUTH0_BASE_URL must be set");

    let auth_url = AuthUrl::new(format!("{}/authorize", auth0_base_url))
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(format!("{}/oauth/token", auth0_base_url))
        .expect("Invalid token endpoint URL");

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool: types::DBPool = establish_connection();

    // let app_state = web::Data::new(types::AppState {
    //     oauth: get_oauth_client(),
    //     tera: tera,
    // });
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();
        let igdb_client = web::Data::new(igdb::IGDB::new().unwrap());
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .data(pool.clone())
            .data(tera)
            .data(get_oauth_client())
            .app_data(igdb_client.clone())
            .route("/", web::get().to(routes::home))
            .route("/login", web::get().to(routes::login))
            .service(
                web::scope("/api")
                    .route("/search", web::get().to(routes::search_igdb_games))
                    .route("/wishlist", web::post().to(routes::add_games_to_wishlist))
            )
            .service(Files::new("/static", "./static"))
            .service(
                web::resource("/login/callback")
                    .name("login_callback")
                    .route(web::get().to(routes::login_callback)),
            )
            .wrap(Logger::default())
    })
    .workers(4) // <- Start 4 workers
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .await
}
