use reqwest;
use std::env;
use anyhow::Context;
use serde_json::Value;
use crate::structure::exploit_routes;

pub async fn get_google_routes(depart: &str, arrivee: &str, modes: &[String]) -> Result<Vec<String>, anyhow::Error> {
    dotenv::dotenv().expect("Erreur de chargement du fichier .env");
    let route_api_key = env::var("ROUTE_API_KEY").expect("ROUTE_API_KEY doit être défini");
    let mut routes: Vec<String> = Vec::new();

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
        let travel = exploit_routes(_response, String::from(mode));
    }

    Ok(routes)
}