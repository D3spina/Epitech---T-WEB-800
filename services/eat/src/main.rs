use serde_json::Value;

#[macro_use] extern crate rocket;
extern crate common;
use common::google::Google;


// url static
#[get("/")]
async fn index() -> String {
    let resto = get_google().await;
    format!("{:?}", resto)
}


// ajout de variable dans l'url
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}


async fn get_google() -> Value{
    let mut google = Google::new();
    google.geocoding(String::from("Paris")).await.expect("TODO: panic message");
    let resto: Value = google.nearby_place(String::from("restaurant"), 1000).await.expect("FDP");
    resto
}