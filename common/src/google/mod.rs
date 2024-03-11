use anyhow::Context;
use serde_json::Value;


pub struct Google {
        city: String,
        lat: f64,
        lng: f64
    }

    impl Google {
        pub fn new(ville: String) -> Google {
            Google { city: ville, lat: 0.0, lng: 0.0 }
        }

        pub async fn geocoding(&mut self, adress: &str) -> Result<String, anyhow::Error> {
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
            Ok(format!("{},{}", self.lat, self.lng))
        }

        // vérification que l'api répond bien :)
        /*pub async fn check_api() -> Result<(), anyhow::Error> {
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
        }*/
    }

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_geocoding_1() {
        let mut google = Google::new("Paris".into());
        let result = google.geocoding("Paris").await;
        assert_eq!(result.unwrap(), "48.862725,2.287592");
    }

    #[tokio::test]
    async fn test_geocoding_2() {
        let mut google = Google::new("Paris".into());
        let result = google.geocoding("80 Rue saint george 54000 Nancy").await;
        assert_eq!(result.unwrap(), "48.692558,6.1882608");
    }

    #[tokio::test]
    async fn test_geocoding_3() {
        let mut google = Google::new("Paris".into());
        let result = google.geocoding("fzfezfezfzf").await;
        assert_eq!(result.unwrap(),  "Erreur de géolocalisation.");
    }
}