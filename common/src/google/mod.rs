use anyhow::{Context, Result};
use dotenv_codegen::dotenv;

// Importation des modules
pub mod nearly_place_model;

#[derive(PartialEq, Debug)]
pub struct Google {
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    api_key: String,
}

impl Google {
    // create a new Google object avec la clé API en attribut privé de base.
    // Initialisation
    pub fn new() -> Self {
        let (lat, lng) = (0.0, 0.0);
        Self {
            city: "".to_string(),
            lat,
            lng,
            api_key: dotenv!("GOOGLE_API_KEY")
                .to_string(),
        }
    }

    // Check if Google Place API is UP
    // Return true if the API is UP, False sinon
    // Cette fonction est incluse dans les autres fonctions pour vérifier que l'API est UP avant de faire des requêtes
    pub async fn check_api(&self) -> Result<bool, anyhow::Error> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address=Paris&key={}",
            self.api_key
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .context("Erreur dans l'envoie de la requête")?
            .text()
            .await
            .context("Erreur dans la récupération de la requête")?;

        let v: serde_json::Value = serde_json::from_str(&response)?;
        if let Some(status) = v["status"].as_str() {
            if status == "OK" {
                Ok(true)
            } else {
                Err(anyhow::anyhow!("Erreur dans la connexion API"))
            }
        } else {
            Err(anyhow::anyhow!("Erreur dans la connexion API"))
        }
    }

    // get the latitude and longitude of the city
    // Nous récupérons du front la localisation de la ville ou l'addresse, nous modifions l'objet google avec les attributs.
    pub async fn geocoding(&mut self, ville: String) -> Result<(), anyhow::Error> {
        // Check if the API is up before proceeding
        self.check_api().await?;
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            ville, self.api_key
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
        let v: serde_json::Value = serde_json::from_str(&response)?;
        if let Some(results) = v["results"].as_array() {
            if !results.is_empty() {
                if let Some(geometry) = results[0]["geometry"]["location"].as_object() {
                    self.lat = geometry.get("lat").unwrap().as_f64().unwrap();
                    self.lng = geometry.get("lng").unwrap().as_f64().unwrap();
                    self.city = ville;
                    return Ok(());
                }
            }
        }
        Err(anyhow::Error::msg("Pas de résultat trouvé"))
    }

    // Search for nearby places in the radius of the city. The type of place is given as a parameter. The type of radius is also given as a parameter.
    // The different type of place can be found here: https://developers.google.com/maps/documentation/places/web-service/supported_types
    pub async fn nearby_place(
        &self,
        type_place: String,
        radius: i32,
    ) -> Result<serde_json::Value, anyhow::Error> {
        // Check if the API is up before proceeding
        self.check_api().await?;
        let location = format!("{},{}", self.lat, self.lng);
        let url = format!("https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={}&radius={}&type={}&key={}",
                              location, radius, type_place, self.api_key);
        let client = reqwest::Client::new();
        let _response = client
            .get(url)
            .send()
            .await
            .context("Erreur dans l'envoie de la requête")?
            .json::<serde_json::Value>()
            .await
            .context("Erreur dans la récupération des données")?;
        return Ok(_response);
    }
}

// Tests unitaires
#[cfg(test)]
mod tests {
    use tokio;

    use super::*;

    #[tokio::test]
    // Test pour une ville spécifique
    async fn test_google_1() {
        let expected_google: Google = Google {
            city: String::from("Paris"),
            lat: 48.856614,
            lng: 2.3522219,
            api_key: dotenv!("GOOGLE_API_KEY")
                .to_string(),
        };
        let mut result = Google::new();
        result
            .geocoding(String::from("Paris"))
            .await
            .expect("erreur dans la récupération des données");
        assert_eq!(result, expected_google);
    }

    #[tokio::test]
    // Test pour une addresse spécifique
    async fn test_google_2() {
        let expected_google = Google {
            city: "80 Rue saint george 54000 Nancy".to_string(),
            lat: 48.6924497,
            lng: 6.1881741,
            api_key: dotenv!("GOOGLE_API_KEY")
                .to_string(),
        };
        let mut result = Google::new();
        result
            .geocoding(String::from("80 Rue saint george 54000 Nancy"))
            .await
            .unwrap();
        assert_eq!(result, expected_google);
    }

    #[tokio::test]
    // Test si l'entrée est innexistante
    async fn test_google_3() {
        let mut result = Google::new();
        result
            .geocoding(String::from("efzefzefezfezf"))
            .await
            .unwrap_err();
    }

    #[tokio::test]
    // Test if restaurants is found in the Paris's 1000m
    async fn test_nearby_place_1() {
        let mut google = Google::new();
        let _ = google.geocoding(String::from("Paris")).await;
        let _result = google.nearby_place("restaurant".to_string(), 1000).await;
        assert!(_result.is_ok());
    }

    #[tokio::test]
    // Test if restaurants are found with a complete address
    async fn test_nearby_place_2() {
        let mut google = Google::new();
        google
            .geocoding(String::from("80 Rue saint george 54000 Nancy"))
            .await
            .unwrap();
        let _result = google.nearby_place("restaurant".to_string(), 0).await;
    }

    #[tokio::test]
    // Test if bars are found with a complete address
    // Second test pour tester l'intégration de type_place
    async fn test_nearby_place_3() {
        let mut google = Google::new();
        google
            .geocoding(String::from("80 Rue saint george 54000 Nancy"))
            .await
            .unwrap();
        let _result = google.nearby_place("bars".to_string(), 0).await;
    }
}
