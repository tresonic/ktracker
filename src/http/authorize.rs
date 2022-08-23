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


    // // Here you can check the user credentials from a database
    // if payload.username != "foo" || payload.pass != "bar" {
    //     return Err(AuthError::WrongCredentials);
    // }

    if !db.auth_user(&payload).await {
        return Err(AuthError::WrongCredentials)
    }


    let claims = Claims {
        id: payload.username,
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}