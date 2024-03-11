use anyhow::Context;
use serde_json::Value;
use tokio;


pub struct Google {
        city: String,
        lat: f64,
        lng: f64
    }

    impl Google {
        pub fn new(ville: String) -> Google {
            Google { city: ville, lat: 0.0, lng: 0.0 }
        }

        pub async fn geocoding(&mut self, adress: &str) {
            dotenv::dotenv().ok();

            let api_key = std::env::var("GOOGLE_API_KEY").expect("Erreur dans la récupération de la clé API Google");
            let url = format!(
                "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
                adress, api_key
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
            if let Some(results) = v["results"].as_array() {
                if !results.is_empty() {
                    if let Some(geometry) = results[0]["geometry"]["location"].as_object() {
                        self.lat = geometry.get("lat").unwrap().as_f64().unwrap();
                        self.lng = geometry.get("lng").unwrap().as_f64().unwrap();
                    }
                }
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_api_success() {
        match check_api().await {
            Ok(_) => (),
            Err(e) => error!("Test échoué avec l'erreur : {}", e),
        }
    }

    #[tokio::test]
    async fn test_geocoding_1() {
        assert_eq!(geocoding("Paris").await, "48.862725,2.287592");
    }

    #[tokio::test]
    async fn test_geocoding_2() {
        assert_eq!(geocoding("80 Rue Saint Georges 54000 Nancy").await, "48.692558,6.1882608");
    }

    #[tokio::test]
    async fn test_geocoding_3() {
        assert_eq!(geocoding("ftikudtuj").await, "Erreur de géolocalisation.");
    }
}