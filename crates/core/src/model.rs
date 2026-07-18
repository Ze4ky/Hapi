use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::basic::RequestMethod;

#[derive(Deserialize, Serialize, Clone)]
pub struct RequestInfo {
    pub name: String,
    pub url: String,
    pub method: RequestMethod,
    pub payload: RequestPayload,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RequestPayload {
    pub headers: Value,
    pub body: Value,
}

#[derive(Deserialize, Serialize)]
pub struct RequestResponse {
    pub code: u16,
    pub response_body: Value,
}
