use std::sync::Arc;

mod openapi;
mod routes;

use openapi::api_document::api_docs;

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{Extension, routing::get};
use openapi::api_document::docs_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);

    let mut api = OpenApi::default();

    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();

    let app = ApiRouter::new()
        .route("/", get(|| async { "Hello world" }))
        .nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

    println!("The application is on http://127.0.0.1:5000/ , nothing there by the way.");
    println!("The docs is on http://127.0.0.1:5000/docs");

    axum::serve(listener, app).await.unwrap();
}
