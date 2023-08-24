#[macro_use]
extern crate rocket;
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use schema::rustaceans;

use crate::auth::BasicAuth;

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

#[post("/users", format = "json", data = "<new_rustacean>")]
async fn add_user(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Failed to add new Rustacean");

        json!(result)
    })
    .await
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
