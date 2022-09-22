//! ktracker backend
//!
//! Run with JWT_SECRET environment variable
//! e.g. JWT_SECRET=secret cargo run

use axum::{
    http::{Method, StatusCode},
    routing::{get, get_service, post},
    Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::signal;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::db::database as daba;
use crate::http::{
    authorize::authorize, create_entry::create_entry, create_user::create_user,
    get_meters::get_meters, highscore::highscore,
};

mod auth;
mod db;
mod http;
mod models;

#[derive(StructOpt, Debug)]
#[structopt(name = "ktracker")]
struct Options {
    #[structopt(short, long)]
    deployed: bool,
    #[structopt(short, long)]
    no_frontend: bool,
}

#[tokio::main]
async fn main() {
    let opt = Options::from_args();

    let db = daba::init_db().await;

    let frontend_path = if !opt.deployed {
        "./frontend/build/"
    } else {
        "./frontend/"
    };

    let addr = if !opt.deployed {
        SocketAddr::from(([0, 0, 0, 0], 3000))
    } else {
        SocketAddr::from(([0, 0, 0, 0], 443))
    };
    
    let api_routes: Router = Router::new()
        .route(
            "/api/authorize",
            post(authorize).layer(Extension(db.clone())),
        )
        .route(
            "/api/create_user",
            post(create_user).layer(Extension(db.clone())),
        )
        .route(
            "/api/get_meters",
            get(get_meters).layer(Extension(db.clone())),
        )
        .route(
            "/api/create_entry",
            post(create_entry).layer(Extension(db.clone())),
        )
        .route(
            "/api/highscore",
            get(highscore).layer(Extension(db.clone())),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(vec![
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        );

    let index = frontend_path.to_string() + "index.html";
    let spa_service = get_service(ServeFile::new(index)).handle_error(|_| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    });
    // build our application with a route
    let frontend_routes = Router::new()
        .route("/", spa_service.clone())
        .route("/login", spa_service.clone())
        .route("/register", spa_service.clone())
        .route("/highscore", spa_service.clone())
        .fallback(
            get_service(ServeDir::new(frontend_path)).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }),
        );

    let app = if opt.no_frontend {
        api_routes
    } else {
        Router::new().merge(api_routes).merge(frontend_routes)
    };

    if !opt.deployed {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    } else {
        let config = RustlsConfig::from_pem_file(
            "/etc/letsencrypt/live/jdav-regensburg.de/fullchain.pem",
            "/etc/letsencrypt/live/jdav-regensburg.de/privkey.pem",
        )
        .await
        .unwrap();

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
