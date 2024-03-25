use std::collections::HashMap;
use serde_json::Value;

#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::exploit_json;
use common::google::Google;

<<<<<<< HEAD

#[get("/service/drink/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> String {
    let types = ["lodging", "campground"];
    //crer un vecteur vide
    let mut res = HashMap::new();

    for &element in types.iter() {
        let google = get_google(localisation.clone(), radius, String::from(element)).await;
        let google_result = exploit_json(google).unwrap();
        res.insert(element, google_result);
    }

    let result = serde_json::to_value(&res).unwrap();

=======
// URL pour récupérer les sleep dans un périmétre donné et pour une localisation donnée
#[get("/service/sleep/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> String {
    let sleep = get_google(localisation, radius).await;
    let result = exploit_json(sleep).unwrap();
>>>>>>> a5c390ab384dd4d1ab3bfb1079ff702979a1c6fd
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
    let result =  google
        .nearby_place(types, radius)
        .await
        .expect("Erreur dans la récupération des données dans la fonction get_google");
    result
}


