use serde::{Serialize};
use axum::{http::StatusCode, response::IntoResponse, Json};
use std::borrow::Cow;

#[derive(Serialize)]
pub struct ProblemDetails<'a> {
    pub r#type: Option<Cow<'a, str>>,
    pub title: Cow<'a, str>,
    pub status: u16,
    pub detail: Option<Cow<'a, str>>,
    pub instance: Option<Cow<'a, str>>,
    pub errors: Vec<FieldError<'a>>,
}

#[derive(Serialize)]
pub struct FieldError<'a> {
    pub field: Cow<'a, str>,
    pub message: Cow<'a, str>
}

pub enum ApiError {
    Validation{
        message: &'static str, 
        errors: Vec<FieldError<'static>>
    },
    BadRequest(&'static str)
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Validation { message, ..} => write!(f, "validation error: {message}"), 
            ApiError::BadRequest(m) => write!(f, "bad request: {m}")
        }
    }
}


impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;
        let (status, title, type_url, detail, errors): (StatusCode, &str, Option<&str>, Option<String>, Vec<FieldError>) = match self {
            Validation { message, errors} => (StatusCode::UNPROCESSABLE_ENTITY, "Validation Failed", Some("https://errors.example.com/validation"), Some(message.to_string()), errors),
            BadRequest(m) => (StatusCode::BAD_REQUEST, "Bad Request", None, Some(m.to_string()), vec![])
        };
        let body = ProblemDetails {
            r#type: type_url.map(Cow::from),
            title: Cow::from(title),
            status: status.as_u16(),
            detail: detail.map(Cow::from),
            instance: None,
            errors,
        };
        return (status, Json(body)).into_response();
    }
}

// impl From<sqlx::Error> for ApiError {
//     fn from(err: sqlx::Error) -> Self {
//         eprintln!("DB Error: {err:#}");
//         ApiError::Db
//     }
// }