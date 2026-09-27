use std::collections::HashMap;

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
    pub is_success: bool,
    pub response_headers: HashMap<String, String>,
    pub response_body: Value,
}
