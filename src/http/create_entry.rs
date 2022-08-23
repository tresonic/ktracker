use axum::{Json, Extension, response::IntoResponse, http::StatusCode};

use crate::{models::CreateEntryPayload, auth::Claims, db::database::Database};

pub async fn create_entry(Json(payload): Json<CreateEntryPayload>, claims: Claims, Extension(db): Extension<Database>,) -> impl IntoResponse {
    db.create_entry(&claims.id, &payload).await;

    StatusCode::OK
}