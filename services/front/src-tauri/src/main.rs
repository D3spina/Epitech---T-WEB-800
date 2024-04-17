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

async fn fetch_restaurants(ville: &str, ratio: u32) -> Result<Vec<Restaurant>, reqwest::Error> {
    let url = format!("http://164.90.242.159/service/eat/{}/{}", ville, ratio);
    let response = reqwest::get(url).await?;
    let restaurants = response.json::<Vec<Restaurant>>().await?;
    Ok(restaurants)
}

async fn fetch_localisation(ville: &str) -> Result<Localisation, reqwest::Error> {
    let url = format!("http://164.90.242.159/coord/{}", ville);
    let response = reqwest::get(&url).await?;
    Ok(response.json().await?)
}

#[tauri::command]
async fn get_restaurants(ville: &str, ratio: u32) -> Result<Vec<Restaurant>, String> {
    fetch_restaurants(&ville, ratio)
        .await
        .map_err(|e| e.to_string())
}

/*#[tauri::command]
async fn get_localisation(ville: &str) -> Result<Localisation, reqwest::Error> {
    let loc = fetch_localisation(&ville).await;
    match loc {
        Ok(ref _localisation) => return loc,
        Err(e) => return Err(e),
    }
}*/

#[tauri::command]
async fn get_localisation(ville: &str) -> Result<Localisation, String> {
    fetch_localisation(ville).await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_restaurants, get_localisation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("Tauri application running...");
}
