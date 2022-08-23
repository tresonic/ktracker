//! Example JWT authorization/authentication.
//!
//! Run with
//!
//! ```not_rust
//! JWT_SECRET=secret cargo run -p example-jwt
//! ```

use auth::Claims;
use axum::{
    routing::{get, post}, Router, Extension,
};
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// use once_cell::sync::Lazy;
// use serde::{Deserialize, Serialize};
use std::{net::SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::http::{error::AuthError, authorize::authorize, create_user::create_user, get_meters::get_meters, create_entry::create_entry};
use crate::db::database as daba;

mod auth;
mod models;
mod http;
mod db;

// Quick instructions
//
// - get an authorization token:
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -d '{"client_id":"foo","client_secret":"bar"}' \
//     http://localhost:3000/authorize
//
// - visit the protected area using the authorized token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
//
// - try to visit the protected area using an invalid token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer blahblahblah' \
//     http://localhost:3000/protected

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_jwt=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = daba::init_db().await;
    
    let app = Router::new()
        .route("/protected", get(protected).layer(Extension(db.clone())))
        .route("/authorize", post(authorize).layer(Extension(db.clone())))
        .route("/create_user", post(create_user).layer(Extension(db.clone())))
        .route("/get_meters", get(get_meters).layer(Extension(db.clone())))
        .route("/create_entry", post(create_entry).layer(Extension(db.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}



