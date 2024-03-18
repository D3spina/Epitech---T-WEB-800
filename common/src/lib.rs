

pub mod google;
pub mod webserver;


pub trait WebServerExt {
    fn add_route(&self) -> warp::filters::BoxedFilter<(warp::http::Response<warp::hyper::Body>,)>;
}

/*#[tokio::test]
async fn test_check_api_success() {
    match check_api().await {
        Ok(_) => (),
        Err(e) => panic!("Test échoué avec l'erreur : {}", e),
    }
}*/
