use super::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime
}

#[derive(Serialize)]
pub struct Game {
    pub id: uuid::Uuid,
    pub title: String,
    pub poster_url: String,
    pub publisher: String,
}
