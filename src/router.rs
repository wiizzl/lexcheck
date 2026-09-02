use crate::handlers::validate_handler;
use axum::{Router, routing::post};

pub fn create_router() -> Router {
    Router::new().route("/validate", post(validate_handler))
}
