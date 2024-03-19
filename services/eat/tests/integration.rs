// TODO : add the crate used
use tokio;

// TODO : Add the crate to use nearbysearch restaurant function

#[tokio::test]
async fn test_nearbysearch_restaurant_1() {
    // TODO : change crate by the crate name
    let result = crate::nearby_restaurant("Paris").await.expect("Échec de la requête de restaurants à proximité");
    assert!(!result.is_empty(), "La liste des restaurants ne devrait pas être vide");
}
#[tokio::test]
async fn test_nearbysearch_restaurant_2() {
    // TODO : change crate by the crate name
    let result = crate::nearby_restaurant("80 Rue SainT Georges 54000 NANcy").await.expect("Échec de la requête de restaurants à proximité");
    assert!(!result.is_empty(), "La liste des restaurants ne devrait pas être vide");
}