use crate::web_server::state::AppState;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

pub async fn rate_limit(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();
    // let ip = parts
    //     .remote_addr
    //     .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
    //     .ip();
    // let redis_key = format!("rate_limit:{}", ip);
    // let redis_connection = state.get_redis();
    // let redis_value: i32 = redis::cmd("INCR")
    //     .arg(redis_key)
    //     .query(redis_connection.get().await.unwrap())
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // if redis_value > 10 {
    //     return Err(StatusCode::TOO_MANY_REQUESTS);
    // }
    // let redis_value: i32 = redis::cmd("EXPIRE")
    //     .arg(redis_key)
    //     .arg(60)
    //     .query(redis_connection.get().await.unwrap())
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // let ip = parts.extensions.get::<ConnectInfo<SocketAddr>>();

    // println!("{:#?}", ip);

    let request = Request::from_parts(parts, body);

    Ok(next.run(request).await)
}
