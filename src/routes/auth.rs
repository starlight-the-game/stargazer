use std::sync::Arc;

use crate::auth::auth_backend::{AuthSessionSimple, PrismaBackend};
use crate::dto::login::Login;
use crate::dto::register::Register;
use crate::openapi::api_result::ApiMessage;
use crate::prisma;
use crate::prisma::PrismaClient;
use aide::NoApi;
use aide::axum::routing::{get_with, post_with};
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::transform::TransformOperation;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::login_required;
use password_auth::generate_hash;
use prisma_client_rust::chrono;
use serde_json::{Value, json};

type Database = Extension<Arc<PrismaClient>>;

// For future me:
// https://github.com/tamasfe/aide/issues/67
// TL;DR: Wrap non-API variables with NoApi(mut x) with type NoApi<T>

pub fn configure_auth_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/logout", get_with(logout, logout_docs))
        .route_layer(login_required!(PrismaBackend))
        .api_route("/register", post_with(register, register_docs))
        .api_route("/login", post_with(login, login_docs))
}

fn register_docs(op: TransformOperation) -> TransformOperation {
    op.description("Register an user.")
        .response_with::<{ StatusCode::OK.as_u16() }, Json<ApiMessage>, _>(|res| {
            res.description("Registration successful.")
        })
        .response_with::<{ StatusCode::BAD_REQUEST.as_u16() }, Json<ApiMessage>, _>(|res| {
            res.description("Registration failed.")
        })
        .tag("Authentication")
}
async fn register(db: Database, Json(register): Json<Register>) -> impl IntoApiResponse {
    let user = db
        .player()
        .find_first(vec![prisma::player::email::equals(register.email.clone())])
        .exec()
        .await
        .expect("Error finding user");

    let msg = ApiMessage::default();

    match user {
        Some(_) => {
            let bad_message = msg
                .clone()
                .with_status(StatusCode::BAD_REQUEST)
                .with_message("Unable to create account.")
                .with_details(Value::from(json!({
                    "message": "There exists another account with provided credentials."
                })));

            (StatusCode::BAD_REQUEST, bad_message)
        }
        None => {
            db.player()
                .create(
                    chrono::offset::Utc::now().timestamp(),
                    register.handle.to_string(),
                    register.email.to_string(),
                    generate_hash(&register.password),
                    generate_hash(&register.password),
                    0,
                    1,
                    0,
                    vec![],
                )
                .exec()
                .await
                .expect("Unable to create account.");

            let good_message = msg
                .clone()
                .with_status(StatusCode::OK)
                .with_message("Success.")
                .with_details(Value::from(json!({"message": "Account creation success."})));

            (StatusCode::OK, good_message)
        }
    }
}

fn login_docs(op: TransformOperation) -> TransformOperation {
    op.description("Login into the system.")
        .response_with::<{ StatusCode::OK.as_u16() }, Json<ApiMessage>, _>(|res| {
            res.description("Login successful.")
        })
        .response_with::<{ StatusCode::FORBIDDEN.as_u16() }, Json<ApiMessage>, _>(|res| {
            res.description("Login failure because credentials mismatch.")
        })
        .response_with::<{ StatusCode::INTERNAL_SERVER_ERROR.as_u16() }, Json<ApiMessage>, _>(
            |res| res.description("We fucked up."),
        )
        .tag("Authentication")
}

async fn login(
    NoApi(mut auth_session): NoApi<AuthSessionSimple>,
    login: Json<Login>,
) -> impl IntoApiResponse {
    let msg = ApiMessage::default();

    let user = match auth_session.authenticate(login.0).await {
        Ok(Some(user)) => user,
        _ => {
            let err_message = msg
                .clone()
                .with_status(StatusCode::FORBIDDEN)
                .with_message("Unable to authenticate.")
                .with_details(json!({
                    "message": "The provided credentials are incorrect."
                }));

            return (StatusCode::FORBIDDEN, err_message);
        }
    };

    if auth_session.login(&user).await.is_err() {
        let err_message = msg
            .clone()
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_message("Unable to authenticate.")
            .with_details(json!({
                "message": "Server fucked up."
            }));

        return (StatusCode::INTERNAL_SERVER_ERROR, err_message);
    }

    let ok_message = msg
        .clone()
        .with_status(StatusCode::OK)
        .with_message("Success.")
        .with_details(json!({
            "message": "Authentication success."
        }));

    (StatusCode::OK, ok_message)
}

fn logout_docs(op: TransformOperation) -> TransformOperation {
    op.description("Logout from the system.")
        .response_with::<{ StatusCode::OK.as_u16() }, Json<ApiMessage>, _>(|res| {
            res.description("Logout successful.")
        })
        .response_with::<{ StatusCode::UNAUTHORIZED.as_u16() }, Json<ApiMessage>, _>(|res| {
            res.description("Did you login?")
        })
        .response_with::<{ StatusCode::INTERNAL_SERVER_ERROR.as_u16() }, Json<ApiMessage>, _>(
            |res| res.description("We fucked up."),
        )
        .tag("Authentication")
}

async fn logout(NoApi(mut auth_session): NoApi<AuthSessionSimple>) -> impl IntoApiResponse {
    let msg = ApiMessage::default();

    if auth_session.logout().await.is_err() {
        let err_message = msg
            .clone()
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_message("Server error.")
            .with_details(json!({
                "message": "We fucked up."
            }));

        return (StatusCode::INTERNAL_SERVER_ERROR, err_message);
    }

    let ok_message = msg
        .clone()
        .with_status(StatusCode::OK)
        .with_message("Success.")
        .with_details(json!({
            "message": "Logged out."
        }));

    (StatusCode::OK, ok_message)
}
