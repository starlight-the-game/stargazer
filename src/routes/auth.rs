use crate::openapi::api_error::ApiError;
use aide::axum::routing::{get_with, post_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::http::StatusCode;
use axum::Json;

pub fn configure_auth_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/register", post_with(register, register_docs))
        .api_route("/login", post_with(login, login_docs))
        .api_route("/logout", get_with(logout, logout_docs))
}

fn register_docs(op: TransformOperation) -> TransformOperation {
    op.description("Register an user.")
        .response::<{ StatusCode::OK.as_u16() }, ()>()
        .response::<{ StatusCode::FORBIDDEN.as_u16() }, Json<ApiError>>()
        .tag("Authentication")
}
async fn register() {}

fn login_docs(op: TransformOperation) -> TransformOperation {
    op.description("Login into the system.")
        .response::<{ StatusCode::OK.as_u16() }, ()>()
        .response::<{ StatusCode::NOT_FOUND.as_u16() }, Json<ApiError>>()
        .response::<{ StatusCode::UNAUTHORIZED.as_u16() }, Json<ApiError>>()
        .tag("Authentication")
}

async fn login() {}

fn logout_docs(op: TransformOperation) -> TransformOperation {
    op.description("Logout from the system.")
        .response::<{ StatusCode::OK.as_u16() }, ()>()
        .response::<{ StatusCode::UNAUTHORIZED.as_u16() }, Json<ApiError>>()
        .tag("Authentication")
}

async fn logout() {}
