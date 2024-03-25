use std::collections::HashMap;
use serde_json::Value;

#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::exploit_json;
use common::google::Google;


#[get("/service/drink/<localisation>/<radius>")]
/*async fn index(localisation: String, radius: i32) -> String {
    let bar = get_google(localisation, radius, types).await;
    let result = exploit_json(bar).unwrap();
    format!("{:?}", result)
}*/
async fn index(localisation: String, radius: i32) -> String {
    let types = ["bar", "cafe", "night_club"];
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
    let result =  google
        .nearby_place(types, radius)
        .await
        .expect("Erreur dans la récupération des données dans la fonction get_google");
    result
}


