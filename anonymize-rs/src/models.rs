use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AnonymizeRequest {
    pub text: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct DeAnonymizeResponse {
    pub text: String,
}
