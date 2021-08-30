use super::schema::{game_stores};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, AsChangeset, PartialEq)]
#[table_name = "game_stores"]
pub struct GameStore {
    pub id: uuid::Uuid,
    pub store_name: String,
    pub added_on: chrono::NaiveDateTime,
    pub updated_on: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub store_user_name: String,
}
