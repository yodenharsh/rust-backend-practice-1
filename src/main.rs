#[macro_use]
extern crate rocket;
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use models::Rustacean;
use rocket::{
    response::status,
    serde::json::{json, Value},
};
use schema::rustaceans;

use crate::auth::BasicAuth;

extern crate base64;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = rustaceans::table
            .limit(100)
            .load::<Rustacean>(c)
            .expect("Failed to read");
        json!(result)
    })
    .await
}

#[post("/users", format = "json")]
fn add_user(_auth: BasicAuth, _db: DbConn) -> Value {
    json!({"get":"gol"})
}

#[put("/users/<id>")]
fn put_user(id: i32, _auth: BasicAuth, _db: DbConn) -> Value {
    json!({"id": id})
}

#[delete("/users")]
fn delete_users(_auth: BasicAuth, _db: DbConn) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![get_users, put_user, add_user, delete_users])
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
