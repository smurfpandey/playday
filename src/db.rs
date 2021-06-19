use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{User, WishedGame};

pub fn get_user_by_email(db_conn: &PgConnection, user_email: &str) -> Result<Option<User>, diesel::result::Error> {
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

pub fn update_user_login_time(db_conn: &PgConnection, user_id: Uuid) -> Result<bool, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let target = users.filter(id.eq(user_id));
    diesel::update(target).set(last_login.eq(Utc::now().naive_utc())).execute(db_conn)?;

    Ok(true)
}

pub fn create_user(db_conn: &PgConnection, new_user: &User) -> Result<bool, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(new_user).execute(db_conn)?;

    Ok(true)
}

pub fn add_games_to_wishlist(db_conn: &PgConnection, games: &Vec<WishedGame>) -> Result<bool, diesel::result::Error> {
    use crate::schema::wished_games::dsl::*;

    diesel::insert_into(wished_games).values(games).execute(db_conn)?;

    Ok(true)
}
