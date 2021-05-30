use super::schema::users;
use diesel::{Queryable, Insertable};
use serde::Deserialize;

#[derive(Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime
}
