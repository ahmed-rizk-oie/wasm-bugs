wit_bindgen::generate!({
    generate_all
});

use crate::exports::mycom::hello2::ratingcoordinator::{Guest, RatingProcessRequest, RatingResponse};
use wasi::http::types::*;

struct RatingCoordinator;

impl Guest for RatingCoordinator {
    fn handle_rating_process(_request: RatingProcessRequest, _response_out: Fields)->RatingResponse {
        RatingResponse{
            name: "".to_string()
        }
       
    }
}

export!(RatingCoordinator);
