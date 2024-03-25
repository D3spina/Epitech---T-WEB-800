use serde::{Serialize, Deserialize};
use serde_json::Value;

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

pub struct RouteInformations {
    pub travel_mode: String,
    pub distance: String,
    pub duration: String,
    pub start_address: String,
    pub end_address: String,
    pub steps: Vec<Step>,
}

impl RouteInformations {
    pub fn new(travel_mode: String, distance: String, duration: String, start_address: String, end_address: String, steps: Vec<Step>) -> Self {
        Self {
            travel_mode,
            distance,
            duration,
            start_address,
            end_address,
            steps,
        }
    }
}

pub fn exploit_routes(value: Value, mode: String) -> RouteInformations {
    let data: Root = serde_json::from_value(value.clone())?;
    let first_route = data.routes.unwrap_or_else(|| vec![]).into_iter().next().expect("No routes found");

    let legs = first_route.legs.unwrap_or_else(|| vec![]).into_iter().next().expect("No legs found");

    let travel_mode = legs.steps.as_ref().unwrap_or(&vec![]).first().map_or("".to_string(), |step| step.travel_mode.clone().unwrap_or_default());
    let distance = legs.distance.as_ref().map_or("".to_string(), |distance| distance.text.clone().unwrap_or_default());
    let duration = legs.duration.as_ref().map_or("".to_string(), |duration| duration.text.clone().unwrap_or_default());
    let start_address = legs.start_address.clone().unwrap_or_default();
    let end_address = legs.end_address.clone().unwrap_or_default();
    let steps = legs.steps.clone().unwrap_or_default();

    let travel = RouteInformations::new {
        travel_mode,
        distance,
        duration,
        start_address,
        end_address,
        steps,
    };

    travel
}

