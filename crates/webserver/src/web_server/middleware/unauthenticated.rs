use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn guest<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    match request.headers().get(header::AUTHORIZATION) {
        Some(_) => Err(StatusCode::FORBIDDEN),
        None => Ok(next.run(request).await),
    }
}
