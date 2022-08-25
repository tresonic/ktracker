use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{auth::Claims, db::database::Database};

pub async fn highscore(_claims: Claims, Extension(db): Extension<Database>) -> impl IntoResponse {
    let highscore = db.highscore().await;

    (StatusCode::OK, Json(highscore))
}
