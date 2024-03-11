use anyhow::Context;
use serde_json::Value;

pub async fn check_api() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY").context("Erreur dans la récupération de la clé API Google")?;
    let url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address=Paris&key={}",
        api_key
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Erreur dans l'envoie de la requête")?
        .text()
        .await
        .context("Erreur dans la récupération des données")?;

    let v: Value = serde_json::from_str(&response)?;
    if let Some(status) = v["status"].as_str() {
        if status == "OK" {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Erreur dans la récupération des données"))
        }
    } else {
        Err(anyhow::anyhow!("Erreur dans la récupération des données"))
    }
}