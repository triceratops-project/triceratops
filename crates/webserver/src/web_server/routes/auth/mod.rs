use axum::{middleware, routing::post, Router};

use crate::web_server::{
    middleware::{Auth, Guest},
    state::AppState,
};

mod login;
mod logout;
mod register;

pub fn router(state: AppState) -> Router<AppState> {
    let router = Router::new()
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
        );

    return router;
}
