use rocket::{Data, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use prometheus;
use prometheus::{Encoder, HistogramVec, TextEncoder};

lazy_static! {
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "http_request_duration_mlliseconds",
        "HTTP request latencies in ms",
        &["handler"]
    ).unwrap();
}

pub struct PrometheusMiddleware;

impl Fairing for PrometheusMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Prometheus Middleware",
            kind: Kind::Request | Kind::Response
        }
    }

    fn on_request(&self, request: &mut Request, data: &Data) {
        let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();
        println!("request");
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        // let mut buffer = vec![];
    }
}

#[get("/metrics")]
pub fn metrics () -> String {
    let metric_familys = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new(); 

    encoder.encode(&metric_familys, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}
