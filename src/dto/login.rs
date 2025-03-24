use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A register request.
#[derive(Serialize, Clone, Deserialize, JsonSchema)]
pub struct Login {
    /// Your email.
    pub email: String,

    /// Your password.
    pub password: String,
}
