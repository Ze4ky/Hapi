use reqwest::Client;
use serde_json::Value;

use crate::{
    model::{RequestPayload, RequestResponse},
    utils::json_to_headers,
};

pub async fn post_request(
    client: Client,
    url: &str,
    payload: RequestPayload,
) -> Result<RequestResponse, reqwest::Error> {
    let headers = json_to_headers(&payload.headers);
    let response = client
        .post(url)
        .headers(headers)
        .body(payload.body.to_string())
        .send()
        .await?;

    let code = response.status().as_u16();
    let response_body = response.json().await?;
    Ok(RequestResponse {
        code,
        response_body,
    })
}

pub async fn get_request(
    client: Client,
    url: &str,
    req_headers: Value,
) -> Result<RequestResponse, reqwest::Error> {
    let headers = json_to_headers(&req_headers);
    let response = client.get(url).headers(headers).send().await?;
    let code = response.status().as_u16();
    let response_body = response.json().await?;

    Ok(RequestResponse {
        code,
        response_body,
    })
}
