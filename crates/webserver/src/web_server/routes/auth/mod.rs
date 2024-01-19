use crate::web_server::{
    middleware::{Auth, Guest},
    state::AppState,
};
use axum::{middleware, routing::post, Router};

mod login;
mod logout;
mod oauth;
mod register;

pub fn router(state: AppState) -> Router<AppState> {
    

    Router::new()
        .route(
            "/login",
            post(login::handler).layer(middleware::from_fn(Guest)),
        )
        .route(
            "/logout",
            post(logout::handler).layer(middleware::from_fn_with_state(state, Auth)),
        )
        .route(
            "/register",
            post(register::handler).layer(middleware::from_fn(Guest)),
        )
        .nest("/oauth", oauth::router())
}
