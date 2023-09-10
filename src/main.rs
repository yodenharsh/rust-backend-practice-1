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
    http::Status,
    response::status::{self, Custom},
    serde::json::{json, Json, Value},
};

use crate::{auth::BasicAuth, repositories::UserRepository};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::find_all(c, 100)
            .map(|user| json!(user))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/users", format = "json", data = "<new_rustacean>")]
async fn add_user(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::create(c, new_rustacean.into_inner())
            .map(|user| json!(user))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/users/<id>")]
async fn get_user(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_one(c, id)
            .map(|user| json!(user))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/users/<id>", format = "json", data = "<rustacean>")]
async fn put_user(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::update_or_save_one(c, id, rustacean.into_inner())
            .map(|user| json!(user))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/users/<id>")]
async fn delete_user(
    _auth: BasicAuth,
    db: DbConn,
    id: i32,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        UserRepository::delete_one(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
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
