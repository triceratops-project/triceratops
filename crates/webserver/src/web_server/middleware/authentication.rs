use axum::{extract::State, http::Request, middleware::Next, response::Response};
use redis::AsyncCommands;

use crate::web_server::state::AppState;

pub async fn auth<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // let a: () = state.get_redis().set("test", 42).await.unwrap();

    let _a: () = state
        .get_redis()
        .get()
        .await
        .unwrap()
        .set("455", 12)
        .await
        .unwrap();

    next.run(request).await
}
