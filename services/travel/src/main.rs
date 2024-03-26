use std::collections::HashMap;
use serde_json::Value;
use rocket::serde::json::Json;
#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::exploit_json;
use common::google::Google;
use crate::structure::{get_google_routes, RouteInformations};

mod structure;


#[get("/service/travel/<localisation1>/<localisation2>")]
async fn index(localisation1: String, localisation2: String) -> Json<Vec<RouteInformations>> {
    let result = get_google_routes(&localisation1, &localisation2, &["bicycling", "transit", "walking", "driving", "motorcycle"]).await.unwrap();
    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}




