use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A register request.
#[derive(Serialize, Clone, Deserialize, JsonSchema)]
pub struct Register {
    /// Your custom username.
    pub handle: String,

    /// Your email.
    pub email: String,

    /// Your password.
    pub password: String,
}
