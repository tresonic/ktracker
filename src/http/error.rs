use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub enum UserCreateError {
    UsernameTooShort,
    UsernameBad,
    PasswordTooShort,
    PasswortNotGoodEnough,
    UsernameTaken,
}

impl IntoResponse for UserCreateError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UserCreateError::UsernameTooShort => (StatusCode::NOT_ACCEPTABLE, "Username is too short"),
            UserCreateError::PasswordTooShort => (StatusCode::NOT_ACCEPTABLE, "Password is too short"),
            UserCreateError::PasswortNotGoodEnough => (StatusCode::NOT_ACCEPTABLE, "Password is too bad"),
            UserCreateError::UsernameTaken => (StatusCode::NOT_ACCEPTABLE, "Username is already taken"),
            UserCreateError::UsernameBad => (StatusCode::NOT_ACCEPTABLE, "Bad Username"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}