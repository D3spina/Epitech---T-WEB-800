use common::webserver::WebServer;
use warp::Filter;

#[tokio::main]
async fn main() {
    let server = WebServer::new(4000);

    // Ajouter une route simple
    server.add_route(warp::path("hello").map(|| warp::reply::html("<h1>Hello, World!</h1>"))).await;

    // Démarrer le serveur avec les routes ajoutées
    server.run().await;
}
