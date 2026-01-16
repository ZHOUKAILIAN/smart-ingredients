//! Custom middleware

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

/// Add request ID to all requests for tracing
pub async fn request_id_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    req.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    next.run(req).await
}
