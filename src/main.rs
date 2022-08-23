//! ktracker backend
//!
//! Run with JWT_SECRET environment variable
//! e.g. JWT_SECRET=secret cargo run


use auth::Claims;
use axum::{
    routing::{get, post}, Router, Extension,
};
use tokio::signal;
use std::{net::SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::http::{error::AuthError, authorize::authorize, create_user::create_user, get_meters::get_meters, create_entry::create_entry};
use crate::db::database as daba;

mod auth;
mod models;
mod http;
mod db;


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
        // .route("/protected", get(protected).layer(Extension(db.clone())))
        .route("/authorize", post(authorize).layer(Extension(db.clone())))
        .route("/create_user", post(create_user).layer(Extension(db.clone())))
        .route("/get_meters", get(get_meters).layer(Extension(db.clone())))
        .route("/create_entry", post(create_entry).layer(Extension(db.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}





