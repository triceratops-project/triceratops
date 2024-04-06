use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project]
pub struct ExampleFuture<F> {
    #[pin]
    response_future: F,
}

impl<F> ExampleFuture<F> {
    pub fn new(response_future: F) -> Self {
        Self { response_future }
    }
}

impl<F, Response, Error> Future for ExampleFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
{
    type Output = Result<Response, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        this.response_future.poll(cx)
    }
}
