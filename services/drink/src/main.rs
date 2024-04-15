use std::collections::HashMap;
use rocket::serde::json::Json;
use serde_json::Value;

#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::{Emplacement, exploit_json};
use common::google::Google;

#[derive(Serialize, Deserialize, Debug)]
struct CityCoord {
    lat: f64,
    long: f64,
}

impl CityCoord {
    fn new(lat: f64, long: f64) -> Self {
        Self {
            lat,
            long
        }
    }
}

#[get("/coord/<localisation>")]
async fn coord(localisation: String) -> Json<CityCoord> {
    let mut google = Google::new();
    google
        .geocoding(localisation)
        .await
        .expect("geocoding n'a pas été éxécuté correctement");
    let result = CityCoord::new(google.lat, google.lng);
    Json(result)
}

#[get("/service/drink/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> Json<HashMap<String, Vec<Emplacement>>> {
    let types = ["bar", "cafe", "night_club"];
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
    rocket::build().mount("/", routes![index, coord])
}

pub(crate) async fn get_google(localisation: String, radius: i32, types: String) -> Value {
    let mut google = Google::new();
    google
        .geocoding(localisation)
        .await
        .expect("Geocoding n'a pas été éxécuté correctement");
    let result  =  google
        .nearby_place(types, radius)
        .await
        .expect("Erreur dans la récupération des données dans la fonction get_google");
    result
}
