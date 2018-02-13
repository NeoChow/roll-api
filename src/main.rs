#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate chrono;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
extern crate ttml;
extern crate uuid;

pub mod die;
pub mod config;
pub mod cors;
pub mod roll;
pub mod v1;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/v1", routes![v1::roll])
        .attach(config::ConfigMiddleware)
        .attach(cors::CORS)
}

fn main() {
    rocket().launch();
}
