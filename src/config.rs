use rocket::Rocket;
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Debug)]
pub struct Config {
    pub access_control_allow_origin: String,
}

pub struct ConfigMiddleware;

impl Fairing for ConfigMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Config Middleware",
            kind: Kind::Attach
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let access_control_allow_origin = rocket.config().get_str("access_control_allow_origin").unwrap_or("http://localhost:3000").to_string();
        Ok(rocket.manage(Config {
            access_control_allow_origin,
        }))
    }

}
