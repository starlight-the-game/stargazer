use aide::{
    axum::{
        routing::{get, get_with}, ApiRouter,
        IntoApiResponse,
    },
    openapi::OpenApi,
    scalar::Scalar,
    transform::TransformOpenApi,
};
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;

/// Populate the API documentation.
pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Starlight Open API")
        .summary("All endpoints of Starlight, whether you like it or not.")
        .description("Uhhh, yes...?")
}

/// Populate the document routes.
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
            |p| p.tag("Documentation"),
        )
        .route("/private/api.json", get(serve_docs));

    aide::generate::infer_responses(false);

    router
}

/// Create JSON file for OpenAPI.
async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
