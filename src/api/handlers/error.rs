use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::domain::{collections, points};

pub enum Error {
    Unknown(String),
    PathError(axum::extract::rejection::PathRejection),
    JsonError(axum::extract::rejection::JsonRejection),
    CollectionsRepo(collections::Error),
    PointsRepo(points::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Unknown(reason) => (StatusCode::INTERNAL_SERVER_ERROR, reason),
            Error::PathError(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            Error::JsonError(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            Error::CollectionsRepo(error) => (StatusCode::NOT_FOUND, error.to_string()),
            Error::PointsRepo(error) => (StatusCode::NOT_FOUND, error.to_string()),
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

impl From<axum::extract::rejection::PathRejection> for Error {
    fn from(inner: axum::extract::rejection::PathRejection) -> Self {
        Error::PathError(inner)
    }
}

impl From<axum::extract::rejection::JsonRejection> for Error {
    fn from(inner: axum::extract::rejection::JsonRejection) -> Self {
        Error::JsonError(inner)
    }
}

impl From<collections::Error> for Error {
    fn from(inner: collections::Error) -> Self {
        Error::CollectionsRepo(inner)
    }
}

impl From<points::Error> for Error {
    fn from(inner: points::Error) -> Self {
        Error::PointsRepo(inner)
    }
}
