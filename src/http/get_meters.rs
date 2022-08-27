use axum::{Extension, response::IntoResponse, http::StatusCode, Json};

use crate::{auth::Claims, db::database::Database};

pub async fn get_meters(claims: Claims, Extension(db): Extension<Database>,) -> impl IntoResponse {
    let meters = db.get_meters(&claims.id).await;

    (StatusCode::OK, Json(meters))
}