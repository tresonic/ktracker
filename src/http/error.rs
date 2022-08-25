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
            AuthError::WrongCredentials => (StatusCode::OK, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::OK, "Missing credentials"),
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
            UserCreateError::UsernameTooShort => (StatusCode::OK, "Username is too short"),
            UserCreateError::PasswordTooShort => (StatusCode::OK, "Password is too short"),
            UserCreateError::PasswortNotGoodEnough => (StatusCode::OK, "Password is too bad"),
            UserCreateError::UsernameTaken => (StatusCode::OK, "Username is already taken"),
            UserCreateError::UsernameBad => (StatusCode::OK, "Bad Username"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}