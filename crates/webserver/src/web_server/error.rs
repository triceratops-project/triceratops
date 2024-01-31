use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ErrorResponse<T: IntoResponse> {
    NotFound(T),
    BadRequest(T),
    Forbidden(T),
    InternalServerError(T),
}

impl<T> IntoResponse for ErrorResponse<T>
where
    T: IntoResponse + Send + Sync,
{
    fn into_response(self) -> Response {
        let error_response = match self {
            Self::NotFound(inner) => (StatusCode::NOT_FOUND, inner),
            Self::BadRequest(inner) => (StatusCode::BAD_REQUEST, inner),
            Self::Forbidden(inner) => (StatusCode::FORBIDDEN, inner),
            Self::InternalServerError(inner) => (StatusCode::INTERNAL_SERVER_ERROR, inner),
        };

        error_response.into_response()
    }
}
