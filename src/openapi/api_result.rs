use axum::{http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::Debug;

/// A default response for API message.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct ApiMessage {
    /// An error message.
    pub message: String,

    /// The status code.
    ///
    /// Use `axum::http::StatusCode.as_u16()` or `Self.with_status()` whenever possible.
    pub status: u16,

    /// [Optional] Additional details.
    /// Use serde_json::json! macro if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl ApiMessage {
    /// Set the message of this response.
    #[allow(dead_code)]
    pub fn with_message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    /// Set the status code of this response.
    #[allow(dead_code)]
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status.as_u16();
        self
    }

    /// Set additional details of this response.
    #[allow(dead_code)]
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl IntoResponse for ApiMessage {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = StatusCode::from_u16(status).unwrap();
        res
    }
}

impl Default for ApiMessage {
    fn default() -> Self {
        Self {
            message: "Brewing failure.".to_string(),
            details: Option::from(json!({
                "you": "did not plug the cord..."
            })),
            status: StatusCode::IM_A_TEAPOT.as_u16(),
        }
    }
}
