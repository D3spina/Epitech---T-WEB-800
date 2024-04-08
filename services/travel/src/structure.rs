use anyhow::Context;
use serde::{Serialize, Deserialize};
use serde_json::{from_value, Value, Error};
use dotenv_codegen::dotenv;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub geocoded_waypoints: Option<Vec<GeocodedWaypoint>>,
    pub routes: Option<Vec<Route>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeocodedWaypoint {
    pub geocoder_status: Option<String>,
    pub place_id: Option<String>,
    pub types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub bounds: Option<Bounds>,
    pub copyrights: Option<String>,
    pub legs: Option<Vec<Leg>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bounds {
    pub northeast: Option<Coordinate>,
    pub southwest: Option<Coordinate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leg {
    pub distance: Option<Distance>,
    pub duration: Option<Duration>,
    pub end_address: Option<String>,
    pub end_location: Option<Coordinate>,
    pub start_address: Option<String>,
    pub start_location: Option<Coordinate>,
    pub steps: Option<Vec<Step>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Distance {
    pub text: Option<String>,
    pub value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duration {
    pub text: Option<String>,
    pub value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub distance: Option<Distance>,
    pub duration: Option<Duration>,
    pub end_location: Option<Coordinate>,
    pub html_instructions: Option<String>,
    pub polyline: Option<Polyline>,
    pub start_location: Option<Coordinate>,
    pub travel_mode: Option<String>,
    pub maneuver: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Polyline {
    pub points: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteInformations {
    pub travel_mode: String,
    pub distance: String,
    pub duration: String,
    pub start_address: String,
    pub end_address: String,
}

impl RouteInformations {
    pub fn new(travel_mode: String, distance: String, duration: String, start_address: String, end_address: String) -> Self {
        Self {
            travel_mode,
            distance,
            duration,
            start_address,
            end_address,
        }
    }
}

pub fn exploit_routes(value: Value, mode: String) -> Result<RouteInformations, Error> {
    let data: Root = from_value(value.clone())?;
    let first_route = data.routes.unwrap_or_else(|| vec![]).into_iter().next().expect("No routes found");

    let legs = first_route.legs.unwrap_or_else(|| vec![]).into_iter().next().expect("No legs found");

    let distance = legs.distance.as_ref().map_or_else(|| "".to_string(), |distance| distance.text.clone().unwrap_or_default());
    let duration = legs.duration.as_ref().map_or_else(|| "".to_string(), |duration| duration.text.clone().unwrap_or_default());
    let start_address = legs.start_address.clone().unwrap_or_default();
    let end_address = legs.end_address.clone().unwrap_or_default();

    Ok(RouteInformations::new(mode, distance, duration, start_address, end_address))
}

pub async fn get_google_routes(depart: &str, arrivee: &str, modes: &[&str]) -> Result<Vec<RouteInformations>, anyhow::Error> {
    let route_api_key = dotenv!("GOOGLE_API_KEY").to_string();
    let mut routes: Vec<RouteInformations> = Vec::new();

    for mode in modes {
        let url = format!("https://maps.googleapis.com/maps/api/directions/json?origin={}&destination={}&mode={}&key={}", depart, arrivee, mode, route_api_key);

        let client = reqwest::Client::new();
        let _response = client
            .get(url)
            .send()
            .await
            .context("Erreur dans l'envoie de la requête")?
            .json::<Value>()
            .await
            .context("Erreur dans la récupération des données")?;
        let travel = exploit_routes(_response, mode.to_string());
        routes.push(travel.unwrap())
    }

    Ok(routes)
}


