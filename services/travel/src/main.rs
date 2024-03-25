use std::collections::HashMap;
use serde_json::Value;

#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::exploit_json;
use common::google::Google;
mod lib;


#[get("/service/travel/<localisation1>/<localisation2>/")]
async fn index(localisation: String, localisation2: String) -> String {

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}




