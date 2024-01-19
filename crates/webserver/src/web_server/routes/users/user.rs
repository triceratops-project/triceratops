use crate::web_server::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::EntityTrait;
use serde_json::json;
use triceratops_server_entity::users as Users;

pub async fn handler(Path(user_id): Path<String>, State(state): State<AppState>) -> Response {
    let user = Users::Entity::find_by_id(&user_id)
        .one(state.get_pool())
        .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    match user {
        Some(user) => Json(user).into_response(),
        None => {
            (StatusCode::NOT_FOUND, Json(json!({"message": "Not Found"}))).into_response()
        }
    }
}
