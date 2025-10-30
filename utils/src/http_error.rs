// src/http_error.rs
use axum::{http::StatusCode, response::{IntoResponse}, Json};
use serde::Serialize;
use std::{borrow::Cow, fmt};

#[derive(Serialize)]
pub struct ProblemDetails<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Cow<'a, str>>,   // e.g. "https://errors.example.com/validation"
    pub title: Cow<'a, str>,            // short, human-readable summary
    pub status: u16,                    // HTTP status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Cow<'a, str>>,   // long message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Cow<'a, str>>, // request path
    // Field-level errors (for validation)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FieldError<'a>>,
}

#[derive(Serialize)]
pub struct FieldError<'a> {
    pub field: Cow<'a, str>,
    pub message: Cow<'a, str>,
}

pub enum ApiError {
    Validation {
        message: &'static str,
        errors: Vec<FieldError<'static>>,
    },
    BadRequest(&'static str),
    NotFound { resource: &'static str, id: String },
    Conflict(&'static str),
    Unauthorized,
    Forbidden,
    Timeout,
    Db,               // hide DB internals from clients
    Internal,         // generic 5xx
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f) // keep it compact
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Validation { message, .. } => write!(f, "validation error: {message}"),
            ApiError::BadRequest(m) => write!(f, "bad request: {m}"),
            ApiError::NotFound { resource, id } => write!(f, "{resource} not found: {id}"),
            ApiError::Conflict(m) => write!(f, "conflict: {m}"),
            ApiError::Unauthorized => write!(f, "unauthorized"),
            ApiError::Forbidden => write!(f, "forbidden"),
            ApiError::Timeout => write!(f, "request timeout"),
            ApiError::Db => write!(f, "database error"),
            ApiError::Internal => write!(f, "internal server error"),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;
        // map variant -> (StatusCode, title, type_url)
        let (status, title, type_url, detail, errors): (StatusCode, &str, Option<&str>, Option<String>, Vec<FieldError>) = match self {
            Validation { message, errors } => (StatusCode::UNPROCESSABLE_ENTITY, "Validation Failed",
                                               Some("https://errors.example.com/validation"), Some(message.to_string()), errors),
            BadRequest(m) => (StatusCode::BAD_REQUEST, "Bad Request", None, Some(m.to_string()), vec![]),
            NotFound { .. } => (StatusCode::NOT_FOUND, "Not Found", None, None, vec![]),
            Conflict(m) => (StatusCode::CONFLICT, "Conflict", None, Some(m.to_string()), vec![]),
            Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized", None, None, vec![]),
            Forbidden => (StatusCode::FORBIDDEN, "Forbidden", None, None, vec![]),
            Timeout => (StatusCode::REQUEST_TIMEOUT, "Request Timeout", None, None, vec![]),
            Db => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error", None, None, vec![]),
            Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", None, None, vec![]),
        };

        // If you want the request path as instance, you can add a small middleware to set an Extension<String>
        // with the path. For simplicity we omit it here.
        let body = ProblemDetails {
            r#type: type_url.map(Cow::from),
            title: Cow::from(title),
            status: status.as_u16(),
            detail: detail.map(Cow::from),
            instance: None,
            errors,
        };

        (status, Json(body)).into_response()
    }
}


impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        // You can branch on error kinds if you want finer mapping:
        // match err {
        //     sqlx::Error::RowNotFound => ApiError::NotFound { resource: "...", id: "..." },
        //     _ => ApiError::Db,
        // }
        // For now, keep it simple:
        eprintln!("DB error: {err:#}"); // log internally
        ApiError::Db
    }
}