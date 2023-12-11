use axum::{
    body::Body,
    http::{Request, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower::util::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

use crate::web_server::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/*file", get(spa_handler))
}

pub async fn spa_handler(uri: Uri) -> Response {
    println!("uri: {:?}", uri);
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    ServeDir::new("../../apps/panel-svelte/build/")
        .fallback(ServeFile::new("../../apps/panel-svelte/index.html"))
        .append_index_html_on_directories(false)
        .oneshot(req)
        .await
        .into_response()
}
