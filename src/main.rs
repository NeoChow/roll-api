#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;

extern crate chrono;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
extern crate ttml;
extern crate uuid;

pub mod die;
pub mod middleware;
pub mod roll;
pub mod v1;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/v1", routes![
               v1::roll
        ])
        .mount("/", routes![
               middleware::prometheus::metrics
        ])
        .attach(middleware::prometheus::PrometheusMiddleware)
        .attach(middleware::config::ConfigMiddleware)
        .attach(middleware::cors::CorsMiddleware)
}

fn main() {
    rocket().launch();
}
