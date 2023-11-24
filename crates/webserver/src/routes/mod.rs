use axum::Router;

mod servers;

pub async fn route() -> Router<()> {
    let router = Router::new().nest("/servers", servers::router());
    return router;
}
