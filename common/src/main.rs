use common::webserver::WebServer;

#[tokio::main]
async fn main() {
    let server: WebServer = WebServer::new(4000);
    server.run().await;
}
