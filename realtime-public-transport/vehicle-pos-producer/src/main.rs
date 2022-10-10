mod config;
mod kafka_producer;

use crate::{config::Config, kafka_producer::KafkaProducer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::from_env();

    let producer = KafkaProducer::new(&config);
    producer.send_with_rand_key("new payload").await.unwrap();

    Ok(())
}
