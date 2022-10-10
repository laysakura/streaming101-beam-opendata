use std::time::Duration;

use rdkafka::{
    error::KafkaError,
    message::{OwnedMessage, ToBytes},
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

use crate::config::Config;

pub(crate) struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub(crate) fn new(config: &Config) -> Self {
        let producer = Self::producer(config);
        Self {
            producer,
            topic: config.kafka_topic.clone(),
        }
    }

    pub(crate) async fn send_with_rand_key<P>(
        &self,
        payload: &P,
    ) -> Result<(), (KafkaError, OwnedMessage)>
    where
        P: ToBytes + ?Sized,
    {
        let f = self.producer.send(
            FutureRecord::to(&self.topic).key("TODO").payload(payload),
            Duration::from_secs(0),
        );
        f.await.map(|_| {
            log::debug!("send a message to Kafka");
        })
    }

    fn producer(config: &Config) -> FutureProducer {
        ClientConfig::new()
            .set("bootstrap.servers", &config.kafka_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("failed to create Kafka producer")
    }
}
