use std::sync::Arc;

use crate::openapi;

use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get, get_with},
    },
    openapi::{OpenApi, Tag},
    scalar::Scalar,
    transform::TransformOpenApi,
};
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

use openapi::api_error::ApiError;

/// Populate the API documentation.
pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Starlight Open API")
        .summary("All endpoints of Starlight, whether you like it or not.")
        .description("Uhhh, yes...?")
        .tag(Tag {
            name: "starlight".into(),
            description: Some("Dude...".into()),
            ..Default::default()
        })
        .default_response_with::<Json<ApiError>, _>(|res| {
            res.example(ApiError {
                error: "some error happened".to_string(),
                details: None,
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}

/// Populate the
pub fn docs_routes() -> ApiRouter {
    aide::generate::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Scalar::new("/docs/private/api.json")
                    .with_title("Starlight API")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    aide::generate::infer_responses(false);

    router
}

/// Create JSON file for OpenAPI.
async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
