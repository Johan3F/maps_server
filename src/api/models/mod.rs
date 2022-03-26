use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{Responder, Response, Result},
    serde::json::{serde_json::json, Value},
};
use serde::Serialize;

pub struct ApiResponse {
    json: Value,
    status_code: Status,
}

impl ApiResponse {
    pub fn new(json: Value, status_code: Status) -> ApiResponse {
        ApiResponse { json, status_code }
    }

    pub fn new_ok(json: Value) -> ApiResponse {
        ApiResponse::new(json, Status::Ok)
    }

    pub fn new_message(message: &str, status_code: Status) -> ApiResponse {
        ApiResponse::new(json!(ApiResponseMessage::new(message)), status_code)
    }
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status_code)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize)]
struct ApiResponseMessage {
    message: String,
}

impl ApiResponseMessage {
    fn new(message: &str) -> ApiResponseMessage {
        ApiResponseMessage {
            message: message.to_owned(),
        }
    }
}
