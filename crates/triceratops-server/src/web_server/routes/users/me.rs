use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
};
use triceratops_entity::users as Users;

pub async fn handler(Extension(user): Extension<Users::Model>) -> Response {
    Json(user).into_response()
}
