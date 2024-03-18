use anyhow::Context;
use serde_json::Value;

#[derive(PartialEq, Debug)]
pub struct Google {
    city: String,
    lat: f64,
    lng: f64,
}

impl Google {
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
        println!("{:?}", result);
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
        /*
        #[tokio::test]
        // Test is restaurant is found in the Paris's 1000m
        async fn test_nearby_place_1() {
            let mut google = Google::new("Paris".into());
            let _result = google.nearby_place("restaurant", "1000").await;
            assert!(_result.is_ok());
        }*/
}
