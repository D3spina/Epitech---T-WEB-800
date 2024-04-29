use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use serde_json::Value;
use common::db::{ActivityDetails, Db};


#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::{exploit_json, Emplacement};
use common::google::Google;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct CityCoord {
    lat: f64,
    long: f64,
}

#[derive(Deserialize)]
struct TravelRequest {
    email: String,
    activity_name: String,
    address: String,
    city: String,
    description: String,
    transport: String,
    depart: String,
    arrive: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    status: String,
    message: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AccountCreationData {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AuthCredentials {
    email: String,
    password: String,
}

impl CityCoord {
    fn new(lat: f64, long: f64) -> Self {
        Self { lat, long }
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

#[post("/login/auth", format = "application/json", data = "<credentials>")]
async fn auth(
    credentials: Json<AuthCredentials>,
    db: &rocket::State<Db>,
) -> Result<Status, Status> {
    match db.login(&credentials.email, &credentials.password).await {
        Ok(true) => Ok(Status::Ok),
        Ok(false) => Err(Status::Forbidden),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/fetch_activities?<email>&<description>")]
async fn get_activities(email: String, description: String) -> Result<Json<Vec<ActivityDetails>>, Status> {
    let db = Db::new().await.expect("coucou");
    match db.fetch_activity_details(&email, &description).await {
        Ok(activity_details) => Ok(Json(activity_details)),
        Err(e) => {
            eprintln!("Error fetching activity details: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

#[post("/add_activity_travel", data = "<travel_request>")]
async fn add_activity_travel(
    db: &rocket::State<Db>,
    travel_request: Json<TravelRequest>,
) -> Json<ApiResponse> {
    match db.add_activity_and_travel(
        &travel_request.email,
        travel_request.activity_name.clone(),
        travel_request.address.clone(),
        travel_request.city.clone(),
        &travel_request.description,
        &travel_request.transport,
        &travel_request.depart,
        &travel_request.arrive,
    ).await {
        Ok(()) => Json(ApiResponse {
            status: "success".into(),
            message: "Activity and travel added successfully".into(),
        }),
        Err(e) => Json(ApiResponse {
            status: "error".into(),
            message: format!("Failed to add activity and travel: {}", e),
        }),
    }
}

#[post(
    "/create_account",
    format = "application/json",
    data = "<account_data>"
)]
async fn create_account_route(
    account_data: Json<AccountCreationData>,
    db: &rocket::State<Db>,
) -> Result<Status, Status> {
    let result = db
        .create_account(
            &account_data.email,
            &account_data.password,
            &account_data.first_name,
            &account_data.last_name,
        )
        .await;

    match result {
        Ok(true) => Ok(Status::Created),
        Ok(false) => Err(Status::Conflict),
        Err(_) => Err(Status::InternalServerError),
    }
}

// URL pour récupérer les restaurant dans un périmétre donné et pour une localisation donnée
#[get("/service/eat/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> Json<Vec<Emplacement>> {
    let restaurant = get_google(localisation, radius).await;
    let result = exploit_json(restaurant).unwrap();
    Json(result)
}

#[rocket::main]
async fn main() {
    let db = Db::new().await.expect("Failed to create database pool");
    let rocket_instance = rocket::build()
        .manage(db)
        .mount("/", routes![index, coord, auth, create_account_route, add_activity_travel, get_activities]);

    match rocket_instance.launch().await {
        Ok(_) => println!("Server launched successfully."),
        Err(e) => println!("Server failed to launch: {:?}", e),
    }
}

pub(crate) async fn get_google(localisation: String, radius: i32) -> Value {
    let mut google = Google::new();
    google
        .geocoding(localisation)
        .await
        .expect("Geocoding n'a pas été éxécuté correctement");
    let result: Value = google
        .nearby_place(String::from("restaurant"), radius)
        .await
        .expect("Erreur dans la récupération des données de la fonction get_google");
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    // Tester si la fonction get_google fonctionne correctement
    // On vérifie avec de bonnes valeurs si on obtient bien un résultat
    // On ne test pas avec une mauvaise valeur car geocoding gère déjà ce probl!me.
    #[tokio::test]
    async fn test_get_google() {
        let result = get_google(String::from("Paris"), 1000).await;
        assert_eq!(result["status"], "OK");
    }

    // On test la fonction index avec un mock json pour comparer
    #[tokio::test]
    // On sait que la fonction exploit_json et get_google fonctionne correctement
    // On test donc que la fonction index s'éxécute correctement et renvoie le même résultat que les
    // autres fonctions séparément.
    async fn test_index() {
        let result = index(String::from("Le Bardon"), 1000).await;
        let expected = exploit_json(get_google(String::from("Le Bardon"), 1000).await).unwrap();

        // Convertir `result` et `expected` en JSON String pour la comparaison
        let result_str =
            serde_json::to_string(&result.into_inner()).expect("Failed to serialize result");
        let expected_str = serde_json::to_string(&expected).expect("Failed to serialize expected");

        // Comparer les chaînes JSON
        assert_eq!(result_str, expected_str);
    }
}
