use reqwest::Client;

use crate::{
    basic::RequestMethod,
    model::{RequestInfo, RequestResponse},
    request::{get_request, post_request},
};

pub mod basic;
pub mod model;
pub mod request;
pub mod utils;

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    pub async fn request(
        &self,
        request_info: RequestInfo,
    ) -> Result<RequestResponse, reqwest::Error> {
        match request_info.method {
            RequestMethod::POST => {
                let response =
                    post_request(self.client.clone(), &request_info.url, request_info.payload)
                        .await?;
                Ok(response)
            }

            RequestMethod::GET => {
                let response = get_request(
                    self.client.clone(),
                    &request_info.url,
                    request_info.payload.headers,
                )
                .await?;
                Ok(response)
            }
        }
    }
}
