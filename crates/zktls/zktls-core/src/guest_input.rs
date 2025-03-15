use alloy_primitives::Bytes;
use serde::{Deserialize, Serialize};

use crate::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInputResponse {
    pub time: String,
    #[serde(with = "serde_bytes")]
    pub stream: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub random: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,
    #[serde(default)]
    pub filtered_responses_begin: Vec<u64>,
    #[serde(default)]
    pub filtered_responses_length: Vec<u64>,
    #[serde(default)]
    pub filtered_responses: Vec<Bytes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInput {
    pub request: Request,
    pub response: GuestInputResponse,
}
