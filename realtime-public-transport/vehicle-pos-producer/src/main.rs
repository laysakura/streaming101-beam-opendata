mod config;

use std::time::Duration;

use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::from_env();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let produce_future = producer.send(
        FutureRecord::to(&config.kafka_topic)
            .key("some key")
            .payload("some payload"),
        Duration::from_secs(0),
    );
    match produce_future.await {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => println!("Error: {:?}", e),
    }

    Ok(())
}
