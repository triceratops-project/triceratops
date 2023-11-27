use std::net::SocketAddr;

use crate::web_server::state::AppState;
use axum::{
    extract::{ConnectInfo, Path, State},
    response::{Html, IntoResponse, Response},
    Extension, Json, http::StatusCode,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct RequestQuery {
    code: String,
    state: String,
}

#[cfg(not(debug_assertions))]
pub async fn headless_handler() {
    compile_error!("You should have removed the sodding debug handler for OAuth, Max & Hayden need to make front end support 3-legged oauth.");
}

#[cfg(debug_assertions)]
pub async fn headless_handler(Path(provider_id): Path<String>) -> Response {
    Html(format!(
        r#"<script async>
        (async () => {{
    const params = new Proxy(new URLSearchParams(window.location.search), {{
        get: (searchParams, prop) => searchParams.get(prop),
    }});

    const req = await fetch("http://localhost:8080/auth/oauth/{}/callback", {{
        method: "POST",
        headers: {{
            "Content-Type": "application/json",
        }},
        body: JSON.stringify({{
            code: params.code,
            state: params.state,
        }}),
    }});
    }})();

    </script>"#,
        provider_id
    ))
    .into_response()
}

pub async fn handler(
    Path(provider): Path<String>,
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
    Json(body): Json<RequestQuery>,
) -> Response {
    let oauth_provider = match provider.as_str() {
        "discord" => state.get_oauth().discord(),
        "whmcs" => todo!(),
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Invalid OAuth2 Provider"})).into_response(),
            )
                .into_response()
        }
    };

    let mut redis_client = match state.get_redis().get().await {
        Ok(client) => client,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})).into_response(),
            )
                .into_response()
        }
    };

    let csrf_token: String = match redis_client
        .get(format!("{}:{}:csrf_code", connection_info.ip(), provider))
        .await
    {
        Ok(Some(code)) => code,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid or expired state"})).into_response(),
            )
                .into_response()
        }
    };

    if body.state != csrf_token {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid CSRF Token"})).into_response(),
        )
            .into_response();
    }

    let pkce_verifier: String = match redis_client
        .get(format!("{}:{}:pkce_code", connection_info.ip(), provider))
        .await
    {
        Ok(Some(code)) => code,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Authentication Timeout"})).into_response(),
            )
                .into_response()
        }
    };

    let token_response = match oauth_provider
        .exchange_code(AuthorizationCode::new(body.code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})).into_response(),
            )
                .into_response()
        }
    };

    token_response
        .access_token()
        .secret()
        .clone()
        .into_response()
}
