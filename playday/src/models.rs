use super::schema::{game_stores, users, wished_games};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, AsChangeset, PartialEq)]
#[table_name = "game_stores"]
pub struct GameStore {
    pub id: uuid::Uuid,
    pub store_name: String,
    pub store_token: serde_json::Value,
    pub added_on: chrono::NaiveDateTime,
    pub updated_on: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub store_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct WishedGame {
    pub id: uuid::Uuid,
    pub title: String,
    pub igdb_id: i32,
    pub igdb_info: serde_json::Value,
    pub added_on: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub pc_release_date: i64,
}
