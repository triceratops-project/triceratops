use super::ExampleService;
use crate::state::AppState;
use tower::Layer;

#[derive(Clone)]
pub struct ExampleLayer {
    state: AppState,
}

impl ExampleLayer {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for ExampleLayer {
    type Service = ExampleService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ExampleService::new(inner, self.state.clone())
    }
}
