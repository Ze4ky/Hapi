use hapi_core::model::{RequestInfo, RequestResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestTable(pub Vec<RequestInfo>);

#[derive(Deserialize, Serialize)]
pub struct ResultFile(pub Vec<RequestResponse>);
