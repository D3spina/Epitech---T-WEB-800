

<<<<<<< Updated upstream
pub mod google;
pub mod webserver;
=======
    let api_key = std::env::var("GOOGLE_API_KEY")
        .context("Erreur dans la récupération de la clé API Google")?;
    let url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address=Paris&key={}",
        api_key
    );
>>>>>>> Stashed changes


pub trait PrintableStruct{
    fn print(&self);
}

pub trait Service {
    fn add_routes(&self) -> warp::filters::BoxedFilter<(impl warp::Reply, )>;
}

<<<<<<< Updated upstream
=======
#[tokio::test]
async fn test_check_api_success() {
    match check_api().await {
        Ok(_) => (),
        Err(e) => panic!("Test échoué avec l'erreur : {}", e),
    }
}
>>>>>>> Stashed changes
