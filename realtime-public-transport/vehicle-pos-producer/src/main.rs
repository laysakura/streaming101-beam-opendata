mod config;
mod kafka_producer;
mod vehicle_pos_request;

use futures_util::{pin_mut, StreamExt};

use crate::{
    config::Config, kafka_producer::KafkaProducer, vehicle_pos_request::VehiclePosRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::from_env();
    let producer = KafkaProducer::new(&config);

    let request = VehiclePosRequest::new(&config);
    let resp_stream = request.response_stream();
    pin_mut!(resp_stream);
    while let Some(resp) = resp_stream.next().await {
        let payload = resp.into_body();
        producer
            .send_with_rand_key(&payload)
            .await
            .expect("failed to send payload to Kafka");
    }

    Ok(())
}
