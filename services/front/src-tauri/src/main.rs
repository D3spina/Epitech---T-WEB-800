use serde::{Deserialize, Serialize};
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    rating: f32,
    address: String,
    picture: String,
}

async fn fetch_restaurants() -> Result<Vec<Restaurant>, reqwest::Error> {
    let url = "http://164.90.242.159/service/eat/nancy/1000";
    let response = reqwest::get(url).await?;
    let restaurants = response.json::<Vec<Restaurant>>().await?;
    println!("voici les restaurant: {:#?}", restaurants);
    Ok(restaurants)
}

#[tauri::command]
async fn get_restaurants() -> Result<Vec<Restaurant>, String> {
    println!("saluthzbvlebrlebr");
    fetch_restaurants().await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_restaurants])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("coucou");
}
