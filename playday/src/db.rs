
use std::env;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

use crate::models::{User, WishedGame, GameStore};
use crate::types;

pub fn run_migrations(conn: &PgConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn establish_pool_connection() -> types::DBPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to db"))
}

pub fn get_user_by_email(db_conn: &PgConnection, user_email: &str) -> Result<Option<User>, Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(user_email.to_owned()))
        .first::<User>(db_conn)
        .optional()?;

    Ok(user)
}

pub fn update_user_login_time(db_conn: &PgConnection, user_id: Uuid) -> Result<bool, Error> {
    use crate::schema::users::dsl::*;

    let target = users.filter(id.eq(user_id));
    diesel::update(target).set(last_login.eq(Utc::now().naive_utc())).execute(db_conn)?;

    Ok(true)
}

pub fn create_user(db_conn: &PgConnection, new_user: &User) -> Result<bool, Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(new_user).execute(db_conn)?;

    Ok(true)
}

pub fn add_games_to_wishlist(db_conn: &PgConnection, games: &Vec<WishedGame>) -> Result<bool, Error> {
    use crate::schema::wished_games::dsl::*;

    diesel::insert_into(wished_games).values(games)
        .on_conflict((igdb_id, user_id))
        .do_nothing()
        .execute(db_conn)?;

    Ok(true)
}

pub fn get_games_from_wishlist(db_conn: &PgConnection, usr_id: Uuid) -> Result<Vec<WishedGame>, Error> {
    use crate::schema::wished_games::dsl::*;

    let results = wished_games.filter(user_id.eq(usr_id)).load::<WishedGame>(db_conn)?;
    Ok(results)
}

pub fn remove_game_from_wishlist(db_conn: &PgConnection, usr_id: Uuid, game_id: Uuid) -> Result<bool, Error> {
    use crate::schema::wished_games::dsl::*;

    diesel::delete(
        wished_games.filter(id.eq(game_id).and(user_id.eq(usr_id)))
    ).execute(db_conn)?;

    Ok(true)
}

pub fn get_all_wishlist_games(db_conn: &PgConnection) -> Result<Vec<WishedGame>, Error> {
    use crate::schema::wished_games::dsl::*;

    let results = wished_games.load::<WishedGame>(db_conn)?;
    Ok(results)
}

pub fn get_future_wishlist_games(db_conn: &PgConnection) -> Result<Vec<WishedGame>, Error> {
    use crate::schema::wished_games::dsl::*;

    let results = wished_games.filter(pc_release_date.gt(Utc::now().timestamp())).load::<WishedGame>(db_conn)?;
    Ok(results)
}


pub fn save_epicgames_login(db_conn: &PgConnection, game_store: &GameStore) -> Result<bool, Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::game_stores::dsl::*;

    diesel::insert_into(game_stores).values(game_store)
        .on_conflict((store_name, user_id))
        .do_update()
        .set(game_store)
        .execute(db_conn)?;

    Ok(true)
}
