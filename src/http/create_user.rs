use axum::{Json, Extension, response::{IntoResponse, Response}, http::StatusCode};

use crate::{http::error::UserCreateError, models::CreateUserPayload, db::database::Database};

pub async fn create_user(Json(payload): Json<CreateUserPayload>, Extension(db): Extension<Database>,) -> Response {
    // match db.create_user(payload).await {
    //     Some(err) => return err,
    //     None => return StatusCode::CREATED,
    // }

    let user_create_result = db.create_user(payload).await;
    if user_create_result.is_none() {
        return StatusCode::CREATED.into_response();
    } else {
        return user_create_result.unwrap().into_response();
    }
}