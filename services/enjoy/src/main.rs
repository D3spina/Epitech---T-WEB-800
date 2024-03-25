use serde_json::Value;

#[macro_use]
extern crate rocket;
extern crate common;
use std::collections::HashMap;
use common::google::nearly_place_model::exploit_json;
use common::google::Google;

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[get("/service/enjoy/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> String {
    let types = ["museum", "park", "art_gallery","library","shopping_mall","clothing_store","book_store","amusement_park","movie_theater"];
    //crer un vecteur vide
    let mut res = HashMap::new();

    for &element in types.iter() {
        let google = get_google(localisation.clone(), radius, String::from(element)).await;
        let google_result = exploit_json(google).unwrap();
        res.insert(element, google_result);
    }

    let result = serde_json::to_value(&res).unwrap();

    format!("{:?}", result)
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
        .nearby_place(String::from("restaurant"), radius)
        .await
        .expect("Erreur dans la récupération des données dans la fonction get_google");
    resto
}