#[macro_use]
extern crate rocket;

mod auth;

use rocket::{
    response::status,
    serde::json::{json, Value},
};

use crate::auth::BasicAuth;

extern crate base64;

#[get("/users")]
fn get_users(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "Harsh M"}, {"id": 5, "name": "Mario"}])
}

#[post("/users", format = "json")]
fn add_user() -> Value {
    json!({"get":"gol"})
}

#[put("/users/<id>")]
fn put_user(id: i32) -> Value {
    json!({"id": id})
}

#[delete("/users")]
fn delete_users() -> status::NoContent {
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
        .launch()
        .await;
}
