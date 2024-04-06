use crate::{
    state::AppState,
    web_server::{error::ErrorResponse, routes::auth::oauth::OAuthRedirectResponse},
};
use axum::{
    extract::{ConnectInfo, State},
    Extension, Json,
};
use fred::{interfaces::KeysInterface, types::Expiration};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use serde_json::{json, Value};
use std::net::SocketAddr;

pub async fn handler(
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
) -> Result<Json<OAuthRedirectResponse>, ErrorResponse<Json<Value>>> {
    let oauth_provider = state
        .oauth()
        .microsoft()
        .as_ref()
        .ok_or(ErrorResponse::Forbidden(Json(
            json!({"message": "Microsoft is not an enabled provider"}),
        )))?;

    let redis_client = state.cache();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_provider
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let redis_expiry = Expiration::EX(state.config().redis.expiry().clone());

    redis_client
        .set(
            format!("{}:microsoft:pkce_code", connection_info.ip()),
            pkce_verifier.secret(),
            Some(redis_expiry),
            None,
            false,
        )
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    let redis_expiry = Expiration::EX(state.config().redis.expiry().clone());

    redis_client
        .set(
            format!("{}:microsoft:csrf_code", connection_info.ip()),
            csrf_token.secret(),
            Some(redis_expiry),
            None,
            false,
        )
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    Ok(Json(OAuthRedirectResponse::new(auth_url)))
}
