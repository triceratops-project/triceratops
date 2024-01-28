mod layer;
mod service;
mod future;

pub use layer::ExampleLayer;
pub use service::ExampleService;
pub use future::ExampleFuture;

#[derive(Debug, Clone)]
pub struct Example {
    meta_data: String,
}
