use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn guest(request: Request, next: Next) -> Result<Response, StatusCode> {
    match request.headers().get(header::AUTHORIZATION) {
        Some(_) => Err(StatusCode::FORBIDDEN),
        None => Ok(next.run(request).await),
    }
}
