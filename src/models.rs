use super::schema::{users, wished_games};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct WishedGame {
    pub id: uuid::Uuid,
    pub title: String,
    pub added_on: chrono::NaiveDateTime,

    pub igdb_info: serde_json::Value,
}
