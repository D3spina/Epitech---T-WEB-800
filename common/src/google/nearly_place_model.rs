use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct TypePlace {
    html_attributions: Vec<Value>,
    next_page_token: String,
    results: Vec<Place>,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Place {
    business_status: String,
    geometry: Geometry,
    icon: String,
    icon_background_color: String,
    icon_mask_base_uri: String,
    name: String,
    opening_hours: Option<OpeningHours>,
    photos: Vec<Photo>,
    place_id: String,
    plus_code: PlusCode,
    rating: f64,
    reference: String,
    scope: String,
    types: Vec<String>,
    user_ratings_total: u64,
    vicinity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Geometry {
    location: Location,
    viewport: Viewport,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Viewport {
    northeast: Location,
    southwest: Location,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpeningHours {
    open_now: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Photo {
    height: u64,
    html_attributions: Vec<String>,
    photo_reference: String,
    width: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlusCode {
    compound_code: String,
    global_code: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListPlace {
    name: String,
    rating: f64,
    addresse: String,
}

pub fn exploit_json(value: &Value) -> Result<Vec<ListPlace>, anyhow::Error> {
    let data: TypePlace = serde_json::from_value(value.clone())?;
    let mut place_list = Vec::new();
    for place in data.results {
        let place = ListPlace {
            name: place.name,
            rating: place.rating,
            addresse: place.vicinity,
        };
        place_list.push(place);
    }
    Ok(place_list)
}
