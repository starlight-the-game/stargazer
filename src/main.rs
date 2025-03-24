use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{Extension, routing::get};
use openapi::api_document::docs_routes;
use routes::auth::configure_auth_routes;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{Level, debug, error};

mod openapi;
#[allow(dead_code, unused_imports)]
mod prisma;
mod routes;

use openapi::api_document::api_docs;
use crate::prisma::PrismaClient;

#[tokio::main]
async fn main() {
    aide::generate::on_error(|error| {
        error!("{error}");
    });

    aide::generate::extract_schemas(true);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let prisma_client = Arc::new(prisma::new_client()
        .await
        .expect("Failed to create Prisma client")
    );
    let mut api = OpenApi::default();
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();

    let app = ApiRouter::new()
        .route("/", get(|| async { "Hello world" }))
        .nest_api_service("/docs", docs_routes())
        .nest_api_service("/api", configure_auth_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .layer(Extension(prisma_client))
        .layer(TraceLayer::new_for_http());

    debug!("The application is on http://127.0.0.1:5000/");
    debug!("The docs is on http://127.0.0.1:5000/docs");

    axum::serve(listener, app).await.unwrap();
}
