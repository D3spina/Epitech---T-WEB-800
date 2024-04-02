use serde_json::Value;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;
extern crate common;
use std::collections::HashMap;
use common::auth;

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[post("/service/auth/login")]
async fn login(/*localisation: String, radius: i32*/) -> Json<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();

    Json(res)
}

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[post("/service/auth/register")]
async fn register(/*localisation: String, radius: i32*/) -> Json<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();

    Json(res)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login,register])
}