use std::{pin::Pin, time::Duration};

use async_stream::stream;
use futures_core::Stream;
use reqwest::RequestBuilder;
use time::Instant;

use crate::config::Config;

#[derive(Debug)]
pub(crate) struct VehiclePosRequest {
    request: RequestBuilder,
    api_call_freq: Duration,

    prev_api_call_at: Instant,
}

impl VehiclePosRequest {
    pub(crate) fn new(config: &Config) -> Pin<Box<Self>> {
        let client = reqwest::Client::new();
        let request = client
            .get(config.vehicle_pos_api_url.clone())
            .header(
                "Authorization",
                format!("apikey {}", config.vehicle_pos_api_key),
            )
            .header("Accept", config.vehicle_pos_api_mime.clone());

        Box::pin(Self {
            request,
            api_call_freq: config.vehicle_pos_api_call_freq,
            prev_api_call_at: Instant::now() - 2 * config.vehicle_pos_api_call_freq, // hack: to fire the first request quickly
        })
    }

    pub(crate) fn response_stream(
        mut self: Pin<Box<Self>>,
    ) -> impl Stream<Item = VehiclePosResponse> {
        stream! {
            loop {
                if self.prev_api_call_at.elapsed() < self.api_call_freq {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    let resp = self
                        .request
                        .try_clone()
                        .expect("should be clonable ReqestBuilder")
                        .send()
                        .await;

                    self.prev_api_call_at = Instant::now();

                    match resp {
                        Ok(res) => {
                            let status = res.status();
                            if status.is_success() {
                                log::debug!("API request succeeded");
                                yield VehiclePosResponse::from_response(res).await
                            } else if status.is_client_error() {
                                panic!("API request failed due to client error: {:?}", res);
                            } else if status.is_server_error() {
                                log::debug!("API request failed due to server error: {:?}", res);
                            } else {
                                unreachable!()
                            }
                        },
                        Err(e) => log::error!("API request failed: {:?}", e),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct VehiclePosResponse {
    body: Vec<u8>,
}

impl VehiclePosResponse {
    pub(crate) fn into_body(self) -> Vec<u8> {
        self.body
    }

    async fn from_response(response: reqwest::Response) -> Self {
        let body: Vec<u8> = response
            .bytes()
            .await
            .expect("successful response is expected to be converted into bytes")
            .into();
        Self { body }
    }
}
