use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::web_server::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().fallback_service(
        ServeDir::new("../../apps/panel-svelte/build/")
            .fallback(ServeFile::new("../../apps/panel-svelte/build/index.html"))
            .append_index_html_on_directories(false),
    )
}
