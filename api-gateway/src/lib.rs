wit_bindgen::generate!({
    generate_all
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;
use crate::mycom::hello2::ratingcoordinator::{RatingProcessRequest};
use crate::mycom::hello2::ratingcoordinator;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        ratingcoordinator::handle_rating_process(&RatingProcessRequest{name: "".to_string()},Fields::new());
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(b"Hello from Rust!\n")
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

export!(HttpServer);
