use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Utc;
use sea_orm::{EntityTrait, ModelTrait};
use sha2::{Digest, Sha512};
use triceratops_entity::{sessions as Sessions, users as Users};

pub async fn auth(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = request.into_parts();

    let auth_header: TypedHeader<Authorization<Bearer>> = parts
        .extract()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token_parts: Vec<&str> = auth_header.token().split('.').collect();

    if token_parts.len() != 2 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_id = match token_parts.first() {
        Some(session_id) => *session_id,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let session_token = match token_parts.get(1) {
        Some(session_token) => *session_token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let session = Sessions::Entity::find_by_id(session_id)
        .one(state.pool())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let session = match session {
        Some(session) => session,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut hasher = Sha512::new();

    hasher.update(session_token.as_bytes());

    let session_token_hash = hasher.finalize();

    if session.token != base16ct::lower::encode_string(&session_token_hash) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    match session.expires_at {
        Some(session_expiry) => {
            if session_expiry < Utc::now() {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        None => {}
    };

    let user = session
        .find_related(Users::Entity)
        .one(state.pool())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match user {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut request = Request::from_parts(parts, body);

    request.extensions_mut().insert(session);
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
