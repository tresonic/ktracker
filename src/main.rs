//! ktracker backend
//!
//! Run with JWT_SECRET environment variable
//! e.g. JWT_SECRET=secret cargo run


use axum::{
    routing::{get, post}, Router, Extension, http::Method,
};
use axum_server::tls_rustls::RustlsConfig;
use tokio::signal;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use structopt::StructOpt;

use crate::http::{authorize::authorize, create_user::create_user, get_meters::get_meters, create_entry::create_entry, highscore::highscore};
use crate::db::database as daba;

mod auth;
mod models;
mod http;
mod db;

#[derive(StructOpt, Debug)]
#[structopt(name = "ktracker")]
struct Options {
    #[structopt(short, long)]
    deployed: bool,
}


#[tokio::main]
async fn main() {
    let opt = Options::from_args();

    let db = daba::init_db().await;
    
    let app = Router::new()
        // .route("/protected", get(protected).layer(Extension(db.clone())))
        .route("/authorize", post(authorize).layer(Extension(db.clone())))
        .route("/create_user", post(create_user).layer(Extension(db.clone())))
        .route("/get_meters", get(get_meters).layer(Extension(db.clone())))
        .route("/create_entry", post(create_entry).layer(Extension(db.clone())))
        .route("/highscore", get(highscore).layer(Extension(db.clone())))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(vec![axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION]))
        .merge(axum_extra::routing::SpaRouter::new("/assets", "./frontend/"));


    if !opt.deployed {
        tracing::debug!("starting in dev mode");
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 443));
        let config = RustlsConfig::from_pem_file("/etc/letsencrypt/live/quack-nak.de/fullchain.pem", "/etc/letsencrypt/live/quack-nak.de/key.pem").await.unwrap();
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
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





