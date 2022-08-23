use axum::{Extension, response::{Response, IntoResponse}, http::StatusCode, Json};

use crate::{auth::Claims, db::database::Database};

pub async fn get_meters(claims: Claims, Extension(db): Extension<Database>,) -> impl IntoResponse {
    // match db.create_user(payload).await {
    //     Some(err) => return err,
    //     None => return StatusCode::CREATED,
    // }

    // let user_create_result = db.create_user(payload).await;
    // if user_create_result.is_none() {
    //     return StatusCode::CREATED.into_response();
    // } else {
    //     return user_create_result.unwrap().into_response();
    // }

    let meters = db.get_meters(&claims.id).await;


    (StatusCode::OK, Json(meters))
}