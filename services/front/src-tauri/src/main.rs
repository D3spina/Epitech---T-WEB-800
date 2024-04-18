use serde::{Deserialize, Serialize};

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    rating: f32,
    address: String,
    picture: String,
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[derive(Serialize, Deserialize, Debug)]
struct Localisation {
    lat: f32,
    long: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lodging {
    name: String,
    rating: f32,
    address: String,
    picture: String,
    lat: f64,
    long: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    campground: Vec<String>, // Assuming campground has a similar structure or change as needed
    lodging: Vec<Lodging>,
}

async fn fetch_restaurants(ville: &str, radius: i16) -> Result<Vec<Restaurant>, reqwest::Error> {
    let url = format!("http://164.90.242.159/service/eat/{}/{}", ville, radius);
    let response = reqwest::get(url).await?;
    let restaurants = response.json::<Vec<Restaurant>>().await?;
    Ok(restaurants)
}

async fn fetch_localisation(ville: &str) -> Result<Localisation, reqwest::Error> {
    let url = format!("http://164.90.242.159/coord/{}", ville);
    let response = reqwest::get(&url).await?;
    Ok(response.json().await?)
}

async fn fetch_sleep(ville: &str, radius: i16) -> Result<Vec<Lodging>, reqwest::Error> {
    let url = format!("http://157.230.76.245/service/sleep/{}/{}", ville, radius);
    let response = reqwest::get(url).await?.json::<Data>().await?;
    Ok(response.lodging)
}

#[tauri::command]
async fn get_restaurants(ville: &str, radius: i16) -> Result<Vec<Restaurant>, String> {
    fetch_restaurants(&ville, radius)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_sleep(ville: &str, radius: i16) -> Result<Vec<Lodging>, String> {
    fetch_sleep(&ville, radius).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_localisation(ville: &str) -> Result<Localisation, String> {
    fetch_localisation(ville).await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_restaurants,
            get_localisation,
            get_sleep,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("Tauri application running...");
}
