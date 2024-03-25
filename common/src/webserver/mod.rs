use std::sync::Arc;
use warp::{Filter, Reply};
use crate::{PrintableStruct, Service};

#[derive(Debug)]
struct WebServer {
    port: u16,
}

impl WebServer {

    fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&self, services: Vec<Arc<dyn Service>>) {
        let base_route = warp::path::end().map(|| warp::reply::html("Base route"));
        let mut combined_routes: warp::filters::BoxedFilter<(impl Reply,)> = base_route.boxed();
        for service in services {
            let service_routes = service.add_routes();

            combined_routes = combined_routes.or(service_routes).boxed().await;
        }
        warp::serve(combined_routes).run(([127, 0, 0, 1], self.port)).await;
    }
}


impl Service for WebServer {
    fn add_routes(&self) -> warp::filters::BoxedFilter<(impl Reply,)> {
        warp::path!("example" / "path")
            .map(|| warp::reply::html("Hello, world!"))
            .boxed()
    }
}

impl PrintableStruct for WebServer {
    fn print(&self) {
        println!("web server running on port {}", &self.port)
    }
}