use crate::web_server::state::AppState;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn router() -> Router<AppState> {
    let spa_service = ServeDir::new("./apps/panel/build/")
        .fallback(ServeFile::new("./apps/panel/build/index.html"))
        .append_index_html_on_directories(false);

    Router::new().nest_service("/", spa_service)
}
