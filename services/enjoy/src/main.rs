use serde_json::Value;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;
extern crate common;
use std::collections::HashMap;
use common::google::nearly_place_model::{Emplacement, exploit_json};
use common::google::Google;

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[get("/service/enjoy/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> Json<HashMap<String, Vec<Emplacement>>> {
    let types = ["museum", "park", "art_gallery","library","shopping_mall","clothing_store","book_store","amusement_park","movie_theater"];
    let mut res: HashMap<String, Vec<Emplacement>> = HashMap::new();

    for &element in types.iter() {
        let google: Value = get_google(localisation.clone(), radius, String::from(element)).await;
        let google_result: Vec<Emplacement> = exploit_json(google).unwrap();
        res.insert(element.to_string(), google_result);
    }

    Json(res)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

pub(crate) async fn get_google(localisation: String, radius: i32, types: String) -> Value {
    let mut google = Google::new();
    google
        .geocoding(localisation)
        .await
        .expect("Geocoding n'a pas été éxécuté correctement");
    let resto: Value = google
        .nearby_place(types, radius)
        .await
        .expect("Erreur dans la récupération des données dans la fonction get_google");
    resto
}