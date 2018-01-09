#![plugin(rocket_codegen)]
#![feature(plugin)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate chrono;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
extern crate ttml;
extern crate uuid;

pub mod die;
pub mod roll;
pub mod v1;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/v1", routes![v1::roll])
}

fn main() {
    rocket().launch();
}
