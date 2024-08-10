use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppAttestationRequest {
    pub challenge_uuid: String,
    pub challenge_timestamp: String,
    pub value: String,
    pub key_id: String,
    pub app_id: String,
    pub raw_attestation: String,
}
