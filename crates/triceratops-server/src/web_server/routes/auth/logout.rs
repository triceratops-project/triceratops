use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use sea_orm::EntityTrait;
use triceratops_entity::sessions as Sessions;

use crate::state::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Extension(session): Extension<Sessions::Model>,
) -> Response {
    let db_action = Sessions::Entity::delete_by_id(session.id)
        .exec(state.pool())
        .await;

    match db_action {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
