use crate::{state::AppState, web_server::error::ErrorResponse};
use axum::{
    extract::{ConnectInfo, State},
    Extension, Json,
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use redis::{AsyncCommands, SetExpiry, SetOptions};
use serde_json::{json, Value};
use std::net::SocketAddr;

pub async fn handler(
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
) -> Result<Json<Value>, ErrorResponse<Json<Value>>> {
    let oauth_provider = state
        .oauth()
        .microsoft()
        .as_ref()
        .ok_or(ErrorResponse::Forbidden(Json(
            json!({"message": "Microsoft is not an enabled provider"}),
        )))?;

    let mut redis_client = state.cache().get().await.map_err(|_| {
        ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
    })?;

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_provider
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let redis_expiry = SetOptions::default().with_expiration(SetExpiry::EX(60));

    redis_client
        .set_options::<String, &String, ()>(
            format!("{}:microsoft:pkce_code", connection_info.ip()),
            pkce_verifier.secret(),
            redis_expiry,
        )
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    let redis_expiry = SetOptions::default().with_expiration(SetExpiry::EX(60));

    redis_client
        .set_options::<String, &String, ()>(
            format!("{}:microsoft:csrf_code", connection_info.ip()),
            csrf_token.secret(),
            redis_expiry,
        )
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    Ok(Json(json!({"url": auth_url})))
}
