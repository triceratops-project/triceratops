use crate::{
    state::AppState,
    web_server::{error::ErrorResponse, routes::auth::oauth::OAuthRedirectResponse},
};
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
) -> Result<Json<OAuthRedirectResponse>, ErrorResponse<Json<Value>>> {
    let oauth_provider = state
        .oauth()
        .discord()
        .as_ref()
        .ok_or(ErrorResponse::Forbidden(Json(
            json!({"message": "Discord is not an enabled provider"}),
        )))?;

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_provider
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("identify".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut redis_client = state.cache().get().await.map_err(|_| {
        ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
    })?;

    let redis_expiry = SetOptions::default().with_expiration(SetExpiry::EX(60));

    redis_client
        .set_options::<String, &String, ()>(
            format!("{}:discord:pkce_code", connection_info.ip()),
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
            format!("{}:discord:csrf_code", connection_info.ip()),
            csrf_token.secret(),
            redis_expiry,
        )
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    Ok(Json(OAuthRedirectResponse::new(auth_url)))
}
