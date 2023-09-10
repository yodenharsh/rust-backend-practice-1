#[macro_use]
extern crate rocket;
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod repositories;
mod schema;

use models::{NewRustacean, Rustacean};
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};

use crate::{auth::BasicAuth, repositories::UserRepository};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = UserRepository::find_all(c, 100).expect("Failed to read");
        json!(result)
    })
    .await
}

#[post("/users", format = "json", data = "<new_rustacean>")]
async fn add_user(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = UserRepository::create(c, new_rustacean.into_inner())
            .expect("Failed to add new Rustacean");

        json!(result)
    })
    .await
}

#[get("/users/<id>")]
async fn get_user(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let result = UserRepository::find_one(c, id).expect("Failed to fetch Rustacean");
        json!(result)
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<rustacean>")]
async fn put_user(id: i32, _auth: BasicAuth, db: DbConn, rustacean: Json<Rustacean>) -> Value {
    db.run(move |c| {
        let result = UserRepository::update_or_save_one(c, id, rustacean.into_inner())
            .expect("Failed to update rustacean");
        json!(result)
    })
    .await
}

#[delete("/users/<id>")]
async fn delete_user(_auth: BasicAuth, db: DbConn, id: i32) -> status::NoContent {
    db.run(move |c| {
        UserRepository::delete_one(c, id).expect("Couldn't delete user");
        status::NoContent
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_users, put_user, add_user, delete_user, get_user],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
