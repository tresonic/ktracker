use axum::{Json, Extension, response::{IntoResponse, Response}, http::StatusCode};

use crate::{models::CreateUserPayload, db::database::Database};

pub async fn create_user(Json(payload): Json<CreateUserPayload>, Extension(db): Extension<Database>,) -> Response {
    let user_create_result = db.create_user(payload).await;
    if user_create_result.is_none() {
        return StatusCode::CREATED.into_response();
    } else {
        return user_create_result.unwrap().into_response();
    }
}