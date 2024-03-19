pub mod google;
pub mod webserver;

pub trait WebServerExt {
    fn add_route(&self) -> warp::filters::BoxedFilter<(warp::http::Response<warp::hyper::Body>,)>;
}
