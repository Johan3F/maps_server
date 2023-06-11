use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::domain::collections;

pub enum Error {
    Unknown(String),
    CollectionsRepo(collections::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Unknown(reason) => (StatusCode::INTERNAL_SERVER_ERROR, reason),
            Error::CollectionsRepo(error) => (StatusCode::NOT_FOUND, error.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(inner: std::convert::Infallible) -> Self {
        Error::Unknown(inner.to_string())
    }
}

impl From<collections::Error> for Error {
    fn from(inner: collections::Error) -> Self {
        Error::CollectionsRepo(inner)
    }
}
