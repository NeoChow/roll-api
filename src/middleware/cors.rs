use rocket::{Request, Response};
use rocket::State;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;

use middleware::config::Config;

pub struct CorsMiddleware;

impl Fairing for CorsMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "CORS Middleware",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            let config = request.guard::<State<Config>>().unwrap();
            response.set_header(Header::new("Access-Control-Allow-Origin", config.access_control_allow_origin.clone()));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
