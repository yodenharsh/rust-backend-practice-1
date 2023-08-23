use super::schema::rustaceans;

use diesel::prelude::*;
use rocket::serde;

#[derive(serde::Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(serde::Deserialize, Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
