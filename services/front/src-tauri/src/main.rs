// #![windows_subsystem = "windows"]
use serde::{Deserialize, Serialize};
use serde_json::json;

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
struct Bar {
    name: String,
    rating: f32,
    address: String,
    picture: String,
    lat: f64,
    long: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Enjoy {
    name: String,
    rating: f32,
    address: String,
    picture: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AllBar {
    bar: Vec<Bar>,
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

#[tokio::main]
async fn fetch_enjoy(ville: &str, radius: i16) -> Result<Vec<Enjoy>, reqwest::Error> {
    let url = format!("http://64.225.95.53/service/enjoy/{}/{}", ville, radius);
    let response = reqwest::get(url).await?;
    let enjoys = response.json::<Vec<Enjoy>>().await?;
    Ok(enjoys)
}

async fn fetch_all_bar(ville: &str, radius: i16) -> Result<Vec<Bar>, reqwest::Error> {
    let url = format!("http://188.166.194.100/service/drink/{}/{}", ville, radius);
    let response = reqwest::get(url).await?;
    let bars = response.json::<AllBar>().await?;
    Ok(bars.bar)
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
async fn get_bar(ville: &str, radius: i16) -> Result<Vec<Bar>, String> {
    fetch_all_bar(&ville, radius)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_enjoy(ville: &str, radius: i16) -> Result<Vec<Enjoy>, String> {
    fetch_enjoy(&ville, radius).map_err(|e| e.to_string())
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

/*#[tauri::command]
async fn register(
    name: &str,
    last_name: &str,
    email: &str,
    password: &str,
) -> Result<bool, String> {
    match create_account(name, last_name, email, password).await {
        Ok(()) => {
            println!("Account creation successful");
            Ok(true)
        }
        Err(e) => {
            eprintln!("Error posting data: {}", e);
            Err(format!("Error posting data: {}", e))
        }
    }
}*/

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_restaurants,
            get_localisation,
            get_sleep,
            get_bar,
            get_enjoy,
            create_account,
            login_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("Tauri application running...");
}

#[tauri::command]
async fn login_api(email: String, password: String) -> Result<(), String> {
    let client = reqwest::Client::new();

    let data = json!({
        "email": email,
        "password": password
    });

    let response = client
        .post("http://164.90.242.159/login/auth")
        .json(&data)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|err| err.to_string())?;
        println!("Response body: {}", body);
        Ok(())
    } else {
        Err(format!("Failed to login: HTTP {}", response.status()))
    }
}

#[tauri::command]
async fn create_account(
    name: String,
    last_name: String,
    email: String,
    password: String,
) -> Result<(), String> {
    // Changed to return String in error case
    let client = reqwest::Client::new();

    let data = json!({
        "email": email,
        "password": password,
        "first_name": name,
        "last_name": last_name
    });

    let response = client
        .post("http://164.90.242.159/create_account")
        .json(&data)
        .send()
        .await
        .map_err(|e| e.to_string())?; // Map error to string to send a more friendly error

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        println!("Response body: {}", body);
        Ok(())
    } else {
        // Optionally handle different status codes differently
        Err(format!(
            "Failed to create account: HTTP {}",
            response.status()
        ))
    }
}
