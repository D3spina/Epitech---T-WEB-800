use rocket::serde::json::Json;
#[macro_use]
extern crate rocket;
extern crate common;
use crate::structure::{get_google_routes, RouteInformations};

mod structure;


#[get("/service/travel/<localisation1>/<localisation2>/<transport_types>")]
async fn index(localisation1: String, localisation2: String, transport_types: String) -> Json<Vec<RouteInformations>> {
    let transport_types_vec: Vec<&str> = transport_types.split(',').collect();

    let result = get_google_routes(&localisation1, &localisation2, &transport_types_vec).await.unwrap();
    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}




