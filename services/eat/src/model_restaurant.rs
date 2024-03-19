use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Restaurant {
    html_attributions: Vec<serde_json::Value>,
    next_page_token: String,
    pub(crate) results: Vec<Place>,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Place {
    business_status: String,
    geometry: Geometry,
    icon: String,
    icon_background_color: String,
    icon_mask_base_uri: String,
    pub(crate) name: String,
    opening_hours: Option<OpeningHours>,
    photos: Vec<Photo>,
    place_id: String,
    plus_code: PlusCode,
    pub(crate) rating: f64,
    reference: String,
    scope: String,
    types: Vec<String>,
    user_ratings_total: u64,
    pub(crate) vicinity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Geometry {
    location: Location,
    viewport: Viewport,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Viewport {
    northeast: Location,
    southwest: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OpeningHours {
    open_now: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Photo {
    height: u64,
    html_attributions: Vec<String>,
    photo_reference: String,
    width: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PlusCode {
    compound_code: String,
    global_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ListRestaurant {
    name: String,
    rating: f64,
    address: String,
    link: String,
    image: String
}