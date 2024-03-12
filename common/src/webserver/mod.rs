use warp::{self, Filter};

pub struct WebServer {
    port: u16,
}

impl WebServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
        }
    }
    pub async fn run(&self) {
        let routes = warp::any().map(|| warp::reply::html("Server is running"));
        warp::serve(routes).run(([127, 0, 0, 1], self.port)).await;
    }
}
