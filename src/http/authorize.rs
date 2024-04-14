use axum::{Json, Extension};
use jsonwebtoken::{encode, Header};

use crate::auth::{Claims, KEYS};
use crate::db::database::Database;
use crate::http::error::AuthError;
use crate::models::{AuthBody, AuthPayload};


pub async fn authorize(Json(payload): Json<AuthPayload>, Extension(db): Extension<Database>,) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if !db.auth_user(&payload).await {
        return Err(AuthError::WrongCredentials)
    }


    let claims = Claims {
        id: payload.username,
        // Mandatory expiry time as UTC timestamp
        exp: 1717243200, // Saturday, June 1, 2024 12:00:00 PM
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
