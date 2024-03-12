

pub mod google;
pub mod webserver;


pub trait PrintableStruct{
    fn print(&self);
}

/*#[tokio::test]
async fn test_check_api_success() {
    match check_api().await {
        Ok(_) => (),
        Err(e) => panic!("Test échoué avec l'erreur : {}", e),
    }
}*/
