use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

// Suite de structure pour gérer le JSON de Google API Place
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

// Objet Emplacement qui contient les informations qui seront envoyés au front
#[derive(Serialize, Deserialize, Debug)]
pub struct Emplacement {
    name: String,
    rating: f64,
    address: String,
    picture: String,
}

impl Emplacement {
    fn new(name: String, rating: f64, address: String, picture: String) -> Self {
        Self {
            name,
            rating,
            address,
            picture,
        }
    }
}

// Cette fonction sert à récupérer depuis le JSON de l'API Google Place les informations qui nous intéressent
// On récupère le nom, la note, l'adresse et la photo de l'endroit qu'on regroupe dans un vecteur de "Emplacement"
pub fn exploit_json(value: Value) -> Result<Vec<Emplacement>, anyhow::Error> {
    let data: TypePlace = serde_json::from_value(value.clone())?;
    dotenv().expect("Impossible de charger le fichier .env");
    let mut place_list = Vec::new();
    for place in data.results {
        // TODO : La clé API se retrouve dans l'URL. Voir pour sécuriser celà.
        let picture = format!("https://maps.googleapis.com/maps/api/place/photo?photoreference={}&sensor=false&maxheight=1000&maxwidth=1000&key={}", place.photos[0].photo_reference, env::var("GOOGLE_API_KEY")?);
        let place = Emplacement::new(place.name, place.rating, place.vicinity, picture);
        place_list.push(place);
    }
    Ok(place_list)
}
