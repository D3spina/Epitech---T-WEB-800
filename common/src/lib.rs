

pub mod google;
pub mod webserver;


pub trait PrintableStruct{
    fn print(&self);
}

pub trait Service {
    fn add_routes(&self) -> warp::filters::BoxedFilter<(impl warp::Reply, )>;
}

