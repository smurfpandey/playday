#[macro_use]
extern crate diesel;
pub mod schema;

use std::env;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::{AuthUrl, ClientId, ClientSecret, StandardRevocableToken, TokenUrl};
use serde::Serialize;
use tera::{Context, Tera};

mod auth;
mod routes;
mod types;

#[derive(Serialize)]
struct Game {
    title: String,
    poster_url: String,
    publisher: String,
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


fn establish_connection() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut tera_data = Context::new();

    let games = [
        Game {
            title: String::from("Horizon Forbidden West"),
            poster_url: String::from(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2gvu.jpg",
            ),
            publisher: String::from("Sony Interactive Entertainment"),
        },
        Game {
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

fn get_oauth_client() -> types::OAuthClient {
    let client_id =
        ClientId::new(env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set"));
    let client_secret = ClientSecret::new(
        env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set"),
    );
    let auth0_base_url = env::var("AUTH0_BASE_URL").expect("AUTH0_BASE_URL must be set");

    let auth_url = AuthUrl::new(format!("{}/authorize", auth0_base_url)).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(format!("{}/oauth/token", auth0_base_url)).expect("Invalid token endpoint URL");

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool: Pool = establish_connection();

    // let app_state = web::Data::new(types::AppState {
    //     oauth: get_oauth_client(),
    //     tera: tera,
    // });
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(pool.clone())
            .data(tera)
            .data(get_oauth_client())
            .route("/", web::get().to(index))
            .route("/login", web::get().to(routes::login))
            //.route("/api/games", web::get().to(routes::get_games))
            .service(Files::new("/static", "./static"))
            .service(
                web::resource("/login/callback")
                    .name("login_callback")
                    .route(web::get().to(routes::login_callback)),
            )
    })
    .bind("127.0.0.1:8000").expect("Can not bind to port 8000")
    .run()
    .await
}
