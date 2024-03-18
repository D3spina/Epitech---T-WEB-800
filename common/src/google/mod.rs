use anyhow::Context;
use serde_json::Value;

#[derive(PartialEq, Debug)]
pub struct Google {
    city: String,
    lat: f64,
    lng: f64,
}

impl Google {

    //check if Google Place API is UP
    pub async fn check_api() -> Result<(), anyhow::Error> {
        let api_key = "AIzaSyAuFLAY6DH36pKwjlJpGNetrGwx4Lt491E";
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
            .context("Erreur dans la récupération de la requête")?;

        let v: Value = serde_json::from_str(&response)?;
        if let Some(status) = v["status"].as_str() {
            if status == "OK" {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Erreur dans la connexion API"))
            }
        } else {
            Err(anyhow::anyhow!("Erreur dans la connexion API"))
        }
    }


    // create a new Google object with the city name
    pub async fn new(ville: String) -> Result<Self, anyhow::Error> {
        let (lat, lng) = Self::geocoding(ville.clone()).await?;
        Ok(Self {
            city: ville,
            lat,
            lng,
        })
    }

    // get the latitude and longitude of the city
    pub async fn geocoding(ville: String) -> Result<(f64, f64), anyhow::Error> {
        Self::check_api().await.context("Échec lors de la vérification de l'API")?;

        let api_key = "AIzaSyAuFLAY6DH36pKwjlJpGNetrGwx4Lt491E";
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            ville, api_key
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
                    let lat = geometry.get("lat").unwrap().as_f64().unwrap();
                    let lng = geometry.get("lng").unwrap().as_f64().unwrap();

                    return Ok((lat, lng));
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
    ) -> Result<String, anyhow::Error> {
        let api_key = "AIzaSyAuFLAY6DH36pKwjlJpGNetrGwx4Lt491E";

        let location = format!("{},{}", self.lat, self.lng);
        let url = format!("https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={}&radius={}&type={}&key={}",
                              location, radius, type_place, api_key);
        let client = reqwest::Client::new();
        let _response = client
            .get(url)
            .send()
            .await
            .context("Erreur dans l'envoie de la requête")?
            .text()
            .await
            .context("Erreur dans la récupération des données")?;
            return Ok(_response);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_api() {
        let result = Google::check_api().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_google_1() {
        let expected_google = Google {
            city: "Paris".to_string(),
            lat: 48.856614,
            lng: 2.3522219,
        };
        let result = Google::new("Paris".to_string()).await;
        println!("{:?}", result);
        match result {
            Ok(google) => assert_eq!(google, expected_google),
            Err(e) => panic!("Test échoué avec l'erreur: {:?}", e),
        }
    }
    #[tokio::test]
    // Test for a specific address
    async fn test_google_2() {
        let expected_google = Google {
            city: "80 Rue saint george 54000 Nancy".to_string(),
            lat: 48.6924497,
            lng: 6.1881741,
        };
        let result = Google::new("80 Rue saint george 54000 Nancy".to_string()).await;
        match result {
            Ok(google) => assert_eq!(google, expected_google),
            Err(e) => panic!("Test échoué avec l'erreur: {:?}", e),
        }
    }

    #[tokio::test]
    // Test for a non-existing city
    async fn test_google_3() {
        let result = Google::new("efzefzefezfezf".to_string()).await;
        assert!(result.is_err(), "Pas de correspondance de localisation");
    }

    #[tokio::test]
    // Test if restaurants is found in the Paris's 1000m
    async fn test_nearby_place_1() {
        let google = Google::new("Paris".to_string()).await;
        let _result = google.expect("Erreur").nearby_place("restaurant".to_string(), 1000).await;
        assert!(_result.is_ok());
    }

    #[tokio::test]
    // Test if restaurants are found with a complete address
    async fn test_nearby_place_2() {
        let google = Google::new("80 Rue saint george 54000 Nancy".to_string()).await;
        let _result = google.expect("Erreur").nearby_place("restaurant".to_string(), 0).await;
        assert!(_result.is_ok());
    }

    #[tokio::test]
    // Test if bars are found with a complete address
    async fn test_nearby_place_3() {
        let google = Google::new("80 Rue saint george 54000 Nancy".to_string()).await;
        let _result = google.expect("Erreur").nearby_place("restaurant".to_string(), 0).await;
        assert!(_result.is_ok());
    }
}
