use super::ExampleFuture;
use crate::state::AppState;
use axum::body::Body;
use hyper::Request;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct ExampleService<S> {
    inner: S,
    state: AppState,
}

impl<S> ExampleService<S> {
    pub fn new(inner: S, state: AppState) -> Self {
        Self { inner, state }
    }
}

impl<S> Service<Request<Body>> for ExampleService<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ExampleFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        println!("Request routed through middleware");

        ExampleFuture::new(self.inner.call(req))
    }
}
