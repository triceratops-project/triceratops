use crate::state::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::EntityTrait;
use serde_json::json;
use triceratops_server_entity::users as Users;

pub async fn handler(State(state): State<AppState>) -> Response {
    let users = Users::Entity::find().all(state.get_pool()).await;

    let users = match users {
        Ok(users) => users,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    Json(users).into_response()
}
